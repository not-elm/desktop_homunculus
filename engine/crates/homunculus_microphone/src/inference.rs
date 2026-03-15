use std::any::Any;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Instant;

use async_broadcast::Sender;
use tokio_util::sync::CancellationToken;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperState};

use crate::error::InferenceError;
use crate::session::{SharedSttSession, SttEvent};
use crate::vad::ChunkEnvelope;

/// Thread 3: spawn the Whisper inference thread via `tokio::task::spawn_blocking`.
///
/// A monitoring task awaits the blocking handle so that if the thread panics
/// beyond `catch_unwind`, the session transitions to `Error` state instead of
/// remaining stuck in `Listening`.
pub fn spawn_inference_thread(
    ctx: Arc<WhisperContext>,
    chunk_rx: crossbeam_channel::Receiver<ChunkEnvelope>,
    language: String,
    cancel: CancellationToken,
    event_tx: Sender<SttEvent>,
    session: SharedSttSession,
    started_at: Instant,
) {
    let handle = tokio::task::spawn_blocking(move || {
        inference_loop(&ctx, &chunk_rx, &language, &cancel, &event_tx, started_at);
        event_tx.try_broadcast(SttEvent::Stopped).ok();
    });

    tokio::spawn(async move {
        if let Err(join_err) = handle.await {
            tracing::error!("inference thread panicked: {join_err}");
            let mut session = session.0.lock().await;
            session.fail(
                "inference_panic".into(),
                format!("Inference thread panicked: {join_err}"),
            );
        }
    });
}

fn inference_loop(
    ctx: &WhisperContext,
    chunk_rx: &crossbeam_channel::Receiver<ChunkEnvelope>,
    language: &str,
    cancel: &CancellationToken,
    event_tx: &Sender<SttEvent>,
    started_at: Instant,
) {
    #[cfg(feature = "cuda")]
    tracing::info!("Inference: CUDA GPU acceleration enabled");
    #[cfg(feature = "metal")]
    tracing::info!("Inference: Metal GPU acceleration enabled");
    #[cfg(not(any(feature = "cuda", feature = "metal")))]
    tracing::info!("Inference: CPU-only mode (no GPU features enabled)");

    tracing::info!("Inference: n_threads={}", optimal_n_threads());

    let mut prev_seq: Option<u64> = None;
    let mut state = match ctx.create_state() {
        Ok(state) => state,
        Err(e) => {
            tracing::error!("failed to create initial whisper state: {e}");
            return;
        }
    };

    loop {
        if cancel.is_cancelled() {
            break;
        }

        let envelope = match chunk_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(envelope) => {
                let latency_ms = envelope.enqueued_at.elapsed().as_millis();
                let len = envelope.samples.len();
                let secs = len as f64 / 16000.0;
                if let Some(prev) = prev_seq {
                    let gap = envelope.seq.saturating_sub(prev);
                    if gap > 1 {
                        tracing::warn!(
                            "Inference: seq gap detected: prev={prev}, current={}, dropped={}",
                            envelope.seq,
                            gap - 1
                        );
                    }
                }
                prev_seq = Some(envelope.seq);
                tracing::info!(
                    "Inference: received chunk seq={}, {len} samples ({secs:.1}s), queue_latency={latency_ms}ms",
                    envelope.seq
                );
                envelope
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
        };

        let inference_start = Instant::now();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_inference(&mut state, &envelope.samples, language)
        }));
        let inference_ms = inference_start.elapsed().as_millis();
        tracing::info!("Inference: completed in {inference_ms}ms");

        if result.is_err() {
            tracing::warn!("Inference: panic detected, recreating whisper state");
            let recreate_start = Instant::now();
            state = match ctx.create_state() {
                Ok(s) => {
                    let recreate_ms = recreate_start.elapsed().as_millis();
                    tracing::info!("Inference: state recreated in {recreate_ms}ms");
                    s
                }
                Err(e) => {
                    tracing::error!("failed to recreate whisper state after panic: {e}");
                    break;
                }
            };
        }

        handle_inference_result(result, started_at, event_tx);
    }
}

fn handle_inference_result(
    result: Result<Result<Option<(String, String)>, InferenceError>, Box<dyn Any + Send>>,
    started_at: Instant,
    event_tx: &Sender<SttEvent>,
) {
    match result {
        Ok(Ok(Some((text, detected_lang)))) => {
            tracing::info!("Inference: result text={text:?}, lang={detected_lang}");
            let timestamp = started_at.elapsed().as_secs_f64();
            event_tx
                .try_broadcast(SttEvent::Result {
                    text,
                    timestamp,
                    language: detected_lang,
                })
                .ok();
        }
        Ok(Ok(None)) => {
            tracing::debug!("Inference: empty result (no segments)");
        }
        Ok(Err(e)) => {
            tracing::error!("whisper inference error: {e}");
        }
        Err(ref panic_info) => {
            let msg = extract_panic_message(panic_info);
            tracing::error!("whisper inference panic: {msg}");
        }
    }
}

fn extract_panic_message(panic_info: &Box<dyn Any + Send>) -> &str {
    panic_info
        .downcast_ref::<String>()
        .map(|s| s.as_str())
        .or_else(|| panic_info.downcast_ref::<&str>().copied())
        .unwrap_or("unknown panic")
}

/// Minimum samples (1 second at 16kHz) to expand timestamp search space.
/// whisper.cpp does not guarantee 30s padding; short chunks with
/// `single_segment=true` constrain the timestamp search range.
const MIN_INFERENCE_SAMPLES: usize = 16000;

/// Discard results with avg_logprobs below this threshold **and** high
/// no-speech probability.  Upstream Whisper uses `-1.0` as a *fallback*
/// trigger (retry at higher temperature), not a hard discard gate.
/// Japanese greedy decoding produces systematically lower logprobs than
/// English, so `-1.5` is used here as a more conservative discard floor.
const AVG_LOGPROBS_THRESHOLD: f32 = -1.5;

/// Segments with `no_speech_prob` above this value are considered silence.
/// Matches the upstream Whisper default (`no_speech_threshold = 0.6`).
const NO_SPEECH_PROB_THRESHOLD: f32 = 0.6;

fn run_inference(
    state: &mut WhisperState,
    samples: &[f32],
    language: &str,
) -> Result<Option<(String, String)>, InferenceError> {
    let params = create_whisper_params(language);
    let samples = pad_short_chunk(samples);

    state
        .full(params, &samples)
        .map_err(|e| InferenceError::Full(e.to_string()))?;

    if should_discard_low_confidence(state) {
        return Ok(None);
    }

    let text = collect_segment_text(state);
    if text.is_empty() {
        return Ok(None);
    }

    let detected_lang = detect_language(state, language);

    Ok(Some((text, detected_lang)))
}

/// Pad chunks shorter than 1s with silence to ensure sufficient timestamp
/// search space for whisper.cpp's decoder.
fn pad_short_chunk(samples: &[f32]) -> Cow<'_, [f32]> {
    if samples.len() >= MIN_INFERENCE_SAMPLES {
        Cow::Borrowed(samples)
    } else {
        let mut padded = samples.to_vec();
        padded.resize(MIN_INFERENCE_SAMPLES, 0.0);
        Cow::Owned(padded)
    }
}

/// Check avg_logprobs across all segments. Discard if confidence is too low
/// to prevent hallucinated output from being emitted.
fn should_discard_low_confidence(state: &WhisperState) -> bool {
    let n_segments = state.full_n_segments();
    if n_segments == 0 {
        return false;
    }

    for i in 0..n_segments {
        let Some(segment) = state.get_segment(i) else {
            continue;
        };

        let no_speech_prob = segment.no_speech_probability();
        let n_tokens = segment.n_tokens();
        if n_tokens == 0 {
            continue;
        }

        let mut sum_logprobs = 0.0f32;
        let mut content_token_count = 0u32;

        for j in 0..n_tokens {
            if let Some(token) = segment.get_token(j) {
                let data = token.token_data();
                // Skip special tokens (SOT, timestamps, language, etc.)
                if data.id >= 50257 {
                    continue;
                }
                sum_logprobs += data.plog;
                content_token_count += 1;
            }
        }

        if content_token_count == 0 {
            continue;
        }

        let avg_logprobs = sum_logprobs / content_token_count as f32;

        tracing::info!(
            "Inference: segment {i} confidence: avg_logprobs={avg_logprobs:.3}, \
             no_speech_prob={no_speech_prob:.3}, content_tokens={content_token_count}"
        );

        if avg_logprobs < AVG_LOGPROBS_THRESHOLD
            && no_speech_prob > NO_SPEECH_PROB_THRESHOLD
        {
            tracing::info!(
                "Inference: discarding low-confidence result \
                 (avg_logprobs={avg_logprobs:.3} < {AVG_LOGPROBS_THRESHOLD} \
                 AND no_speech_prob={no_speech_prob:.3} > {NO_SPEECH_PROB_THRESHOLD})"
            );
            return true;
        }
    }

    false
}

fn optimal_n_threads() -> i32 {
    let physical = num_cpus::get_physical();
    let n = (physical / 2).clamp(1, 8);
    n as i32
}

fn create_whisper_params<'a>(language: &'a str) -> FullParams<'a, 'a> {
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_suppress_nst(true);
    params.set_single_segment(true);
    params.set_n_threads(optimal_n_threads());
    params.set_no_context(true);
    params.set_temperature_inc(0.2);

    if language == "auto" {
        params.set_language(None);
    } else {
        params.set_language(Some(language));
    }

    params
}

fn collect_segment_text(state: &WhisperState) -> String {
    let n_segments = state.full_n_segments();
    let mut text = String::new();
    for i in 0..n_segments {
        if let Some(segment) = state.get_segment(i)
            && let Ok(segment_text) = segment.to_str()
        {
            text.push_str(segment_text);
        }
    }
    text.trim().to_string()
}

fn detect_language(state: &WhisperState, fallback: &str) -> String {
    let lang_id = state.full_lang_id_from_state();
    whisper_rs::get_lang_str(lang_id)
        .map(String::from)
        .unwrap_or_else(|| fallback.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimal_n_threads_respects_bounds() {
        let n = optimal_n_threads();
        assert!(n >= 1, "n_threads must be at least 1, got {n}");
        assert!(n <= 8, "n_threads must be at most 8, got {n}");
    }
}
