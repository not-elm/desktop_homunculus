use std::any::Any;
use std::sync::Arc;
use std::time::Instant;

use async_broadcast::Sender;
use tokio_util::sync::CancellationToken;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperState};

use crate::error::InferenceError;
use crate::session::SttEvent;

/// Thread 3: spawn the Whisper inference thread via `tokio::task::spawn_blocking`.
pub fn spawn_inference_thread(
    ctx: Arc<WhisperContext>,
    chunk_rx: crossbeam_channel::Receiver<Vec<f32>>,
    language: String,
    cancel: CancellationToken,
    event_tx: Sender<SttEvent>,
    started_at: Instant,
) {
    tokio::task::spawn_blocking(move || {
        inference_loop(&ctx, &chunk_rx, &language, &cancel, &event_tx, started_at);
        event_tx.try_broadcast(SttEvent::Stopped).ok();
    });
}

fn inference_loop(
    ctx: &WhisperContext,
    chunk_rx: &crossbeam_channel::Receiver<Vec<f32>>,
    language: &str,
    cancel: &CancellationToken,
    event_tx: &Sender<SttEvent>,
    started_at: Instant,
) {
    loop {
        if cancel.is_cancelled() {
            break;
        }

        let samples = match chunk_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(samples) => samples,
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
        };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_inference(ctx, &samples, language)
        }));

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
            let timestamp = started_at.elapsed().as_secs_f64();
            event_tx
                .try_broadcast(SttEvent::Result {
                    text,
                    timestamp,
                    language: detected_lang,
                })
                .ok();
        }
        Ok(Ok(None)) => {}
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

fn run_inference(
    ctx: &WhisperContext,
    samples: &[f32],
    language: &str,
) -> Result<Option<(String, String)>, InferenceError> {
    let mut state = ctx
        .create_state()
        .map_err(|e| InferenceError::CreateState(e.to_string()))?;

    let params = create_whisper_params(language);

    state
        .full(params, samples)
        .map_err(|e| InferenceError::Full(e.to_string()))?;

    let text = collect_segment_text(&state);
    if text.is_empty() {
        return Ok(None);
    }

    let detected_lang = detect_language(&state, language);

    Ok(Some((text, detected_lang)))
}

fn create_whisper_params<'a>(language: &'a str) -> FullParams<'a, 'a> {
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_suppress_nst(true);
    params.set_single_segment(true);

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
