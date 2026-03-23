use crossbeam_channel::TrySendError;
use rubato::Resampler;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

/// Wrapper around audio samples with timing metadata for pipeline observability.
pub struct ChunkEnvelope {
    /// The audio samples (16kHz, mono, f32).
    pub samples: Vec<f32>,
    /// When this chunk was enqueued to the inference channel.
    pub enqueued_at: Instant,
    /// When VAD detected silence that triggered this chunk emission.
    pub silence_detected_at: Instant,
    /// Monotonically increasing sequence number for gap detection.
    pub seq: u64,
}

/// Atomic counters for pipeline health monitoring.
pub struct PipelineMetrics {
    drops: AtomicU64,
    seq: AtomicU64,
}

impl PipelineMetrics {
    /// Create a new `PipelineMetrics` with all counters initialized to zero.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            drops: AtomicU64::new(0),
            seq: AtomicU64::new(0),
        })
    }

    /// Increment the drop counter by one.
    pub fn increment_drops(&self) {
        self.drops.fetch_add(1, Ordering::Relaxed);
    }

    /// Return the current drop count.
    pub fn drop_count(&self) -> u64 {
        self.drops.load(Ordering::Relaxed)
    }

    /// Return the next sequence number, incrementing the internal counter.
    pub fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, Ordering::Relaxed)
    }
}

/// VAD configuration.
#[derive(Clone, Debug)]
pub struct VadConfig {
    pub silence_ms: u32,
    pub energy_threshold: f32,
    /// Maximum chunk duration in ms. Chunks exceeding this are force-emitted.
    /// Default: 8000ms (8 seconds).
    pub max_chunk_ms: Option<u32>,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            silence_ms: 300,
            energy_threshold: 0.01,
            max_chunk_ms: Some(8000),
        }
    }
}

impl VadConfig {
    /// Build VAD configuration from the given `SttConfig`, falling back to defaults.
    pub fn from_stt_config(stt: &homunculus_utils::config::SttConfig) -> Self {
        Self {
            silence_ms: stt.silence_ms.unwrap_or(300),
            energy_threshold: stt.energy_threshold.unwrap_or(0.01),
            max_chunk_ms: stt.max_chunk_ms.or(Some(8000)),
        }
    }
}

/// VAD state machine that buffers speech and emits chunks on silence boundaries.
///
/// Operates at the VAD sample rate (which may be the capture device's native rate).
/// When `needs_post_resample` is true, `finalize_chunk` resamples output to 16kHz.
pub struct VadStateMachine {
    speech_buffer: Vec<f32>,
    silence_samples: usize,
    in_speech: bool,
    silence_threshold: usize,
    min_chunk_samples: usize,
    energy_threshold: f32,
    frame_i16_buf: Vec<i16>,
    max_chunk_samples: Option<usize>,
    vad_rate: u32,
    needs_post_resample: bool,
}

impl VadStateMachine {
    /// Create a new state machine with the given configuration.
    ///
    /// `vad_rate` is the sample rate at which VAD operates (e.g. 16000 or 48000).
    /// If `needs_post_resample` is true, finalized chunks are resampled to 16kHz.
    pub fn new(config: &VadConfig, vad_rate: u32, needs_post_resample: bool) -> Self {
        let vad_frame_size = (vad_rate / 100) as usize; // 10ms at vad_rate
        Self {
            speech_buffer: Vec::new(),
            silence_samples: 0,
            in_speech: false,
            silence_threshold: (vad_rate as f64 * config.silence_ms as f64 / 1000.0)
                .min(usize::MAX as f64) as usize,
            min_chunk_samples: (vad_rate as f64 * 0.3) as usize, // 0.3s at vad_rate
            energy_threshold: config.energy_threshold,
            frame_i16_buf: vec![0i16; vad_frame_size],
            max_chunk_samples: config
                .max_chunk_ms
                .map(|ms| (vad_rate as f64 * ms as f64 / 1000.0) as usize),
            vad_rate,
            needs_post_resample,
        }
    }

    /// Process a single VAD frame. Returns a completed speech chunk (16kHz) when
    /// silence is detected.
    ///
    /// Forces emission when the buffer exceeds `max_chunk_samples` to prevent
    /// O(L²) Whisper inference cost from unbounded continuous speech.
    pub fn process_frame(&mut self, frame: &[f32], is_voice: bool) -> Option<Vec<f32>> {
        if is_voice {
            self.in_speech = true;
            self.silence_samples = 0;
            self.speech_buffer.extend_from_slice(frame);

            if let Some(max) = self.max_chunk_samples
                && self.speech_buffer.len() >= max
            {
                return self.finalize_chunk();
            }

            return None;
        }

        if !self.in_speech {
            return None;
        }

        self.silence_samples += frame.len();

        if self.silence_samples >= self.silence_threshold {
            return self.finalize_chunk();
        }

        None
    }

    /// Convert an `f32` frame to `i16` in-place, reusing the internal buffer.
    pub fn convert_frame_to_i16(&mut self, frame: &[f32]) -> &[i16] {
        self.frame_i16_buf.resize(frame.len(), 0);
        for (dst, &src) in self.frame_i16_buf.iter_mut().zip(frame.iter()) {
            *dst = (src.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        }
        &self.frame_i16_buf
    }

    /// Take the buffered speech, reset state, and return the chunk (resampled to
    /// 16kHz if needed) if it meets minimum length and energy requirements.
    pub fn finalize_chunk(&mut self) -> Option<Vec<f32>> {
        let chunk = std::mem::take(&mut self.speech_buffer);
        self.in_speech = false;
        self.silence_samples = 0;

        if chunk.len() < self.min_chunk_samples {
            return None;
        }

        if rms_energy(&chunk) < self.energy_threshold {
            return None;
        }

        if self.needs_post_resample {
            Some(resample_batch(self.vad_rate, &chunk))
        } else {
            Some(chunk)
        }
    }

    /// Force-emit any buffered speech, regardless of silence counter state.
    ///
    /// Used for idle finalization (no audio data arriving) and shutdown flushing.
    /// Returns `None` if not currently in speech or if the chunk fails
    /// min-length / energy checks.
    pub fn flush_speech(&mut self) -> Option<Vec<f32>> {
        if !self.in_speech {
            return None;
        }
        self.finalize_chunk()
    }
}

/// Compute the RMS (root-mean-square) energy of a sample buffer.
pub fn rms_energy(samples: &[f32]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }
    (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
}

/// Convert an `f32` audio frame to `i16`, clamping values to `[-1.0, 1.0]`.
pub fn convert_f32_to_i16(frame: &[f32]) -> Vec<i16> {
    frame
        .iter()
        .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
        .collect()
}

/// Accumulates variable-length audio samples and resamples them in fixed-size
/// chunks via `SincFixedIn`. Returns concatenated resampled output.
struct ResampleAccumulator {
    resampler: rubato::SincFixedIn<f32>,
    buf: Vec<f32>,
}

impl ResampleAccumulator {
    fn new(source_rate: u32) -> Self {
        const CHUNK_SIZE: usize = 1024;
        const NUM_CHANNELS: usize = 1;
        let resampler = rubato::SincFixedIn::<f32>::new(
            16000.0 / source_rate as f64,
            2.0,
            rubato::SincInterpolationParameters {
                sinc_len: 64,
                f_cutoff: 0.95,
                oversampling_factor: 64,
                interpolation: rubato::SincInterpolationType::Linear,
                window: rubato::WindowFunction::BlackmanHarris2,
            },
            CHUNK_SIZE,
            NUM_CHANNELS,
        )
        .expect("failed to create resampler");
        Self {
            resampler,
            buf: Vec::new(),
        }
    }

    /// Accumulate samples, drain and resample in fixed-size chunks, return
    /// all resampled output concatenated into a single buffer.
    fn push(&mut self, samples: &[f32]) -> Vec<f32> {
        self.buf.extend_from_slice(samples);
        let mut output = Vec::new();
        let n = self.resampler.input_frames_next();
        while self.buf.len() >= n {
            let chunk: Vec<f32> = self.buf.drain(..n).collect();
            match self.resampler.process(&[chunk], None) {
                Ok(resampled) => {
                    if let Some(ch) = resampled.into_iter().next() {
                        output.extend(ch);
                    }
                }
                Err(e) => tracing::error!("resample error: {e}"),
            }
        }
        output
    }

    /// Flush any remaining buffered samples using partial processing.
    fn flush(&mut self) -> Vec<f32> {
        if self.buf.is_empty() {
            return Vec::new();
        }
        let partial = std::mem::take(&mut self.buf);
        match self.resampler.process_partial(Some(&[partial]), None) {
            Ok(resampled) => resampled.into_iter().next().unwrap_or_default(),
            Err(e) => {
                tracing::error!("resample flush error: {e}");
                Vec::new()
            }
        }
    }
}

/// Resample a complete audio buffer from `source_rate` to 16kHz in one batch.
fn resample_batch(source_rate: u32, samples: &[f32]) -> Vec<f32> {
    let mut acc = ResampleAccumulator::new(source_rate);
    let mut output = acc.push(samples);
    output.extend(acc.flush());
    output
}

/// Map a capture sample rate to the best webrtc-vad rate.
///
/// Returns `(vad_enum, vad_hz, needs_pre_resample)`. When `needs_pre_resample` is
/// true the caller must resample to 16kHz *before* VAD (the rate is unsupported by
/// webrtc-vad, e.g. 44.1kHz).
fn select_vad_rate(sample_rate: u32, needs_resample: bool) -> (webrtc_vad::SampleRate, u32, bool) {
    if !needs_resample {
        return (webrtc_vad::SampleRate::Rate16kHz, 16000, false);
    }
    match sample_rate {
        8000 => (webrtc_vad::SampleRate::Rate8kHz, 8000, false),
        32000 => (webrtc_vad::SampleRate::Rate32kHz, 32000, false),
        48000 => (webrtc_vad::SampleRate::Rate48kHz, 48000, false),
        _ => (webrtc_vad::SampleRate::Rate16kHz, 16000, true),
    }
}

/// Thread 2: spawn the VAD + chunking thread.
pub fn spawn_vad_thread(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
    metrics: Arc<PipelineMetrics>,
) -> Result<crossbeam_channel::Receiver<ChunkEnvelope>, crate::error::PipelineError> {
    let (chunk_tx, chunk_rx) = crossbeam_channel::bounded::<ChunkEnvelope>(3);

    std::thread::Builder::new()
        .name("stt-vad".into())
        .spawn(move || {
            vad_thread_main(
                audio_rx,
                sample_rate,
                needs_resample,
                cancel,
                config,
                chunk_tx,
                metrics,
            );
        })
        .map_err(|e| crate::error::PipelineError::Vad(e.to_string()))?;

    Ok(chunk_rx)
}

fn vad_thread_main(
    audio_rx: mpsc::Receiver<Vec<f32>>,
    sample_rate: u32,
    needs_resample: bool,
    cancel: CancellationToken,
    config: VadConfig,
    chunk_tx: crossbeam_channel::Sender<ChunkEnvelope>,
    metrics: Arc<PipelineMetrics>,
) {
    let (vad_rate_enum, vad_rate, needs_pre_resample) =
        select_vad_rate(sample_rate, needs_resample);
    let needs_post_resample = needs_resample && !needs_pre_resample;

    let mut pre_accumulator = if needs_pre_resample {
        Some(ResampleAccumulator::new(sample_rate))
    } else {
        None
    };

    let mut vad =
        webrtc_vad::Vad::new_with_rate_and_mode(vad_rate_enum, webrtc_vad::VadMode::VeryAggressive);
    let mut state_machine = VadStateMachine::new(&config, vad_rate, needs_post_resample);
    let vad_frame_size = (vad_rate / 100) as usize;

    let mut first_voice_logged = false;
    let mut first_audio_logged = false;
    let mut sample_buf: Vec<f32> = Vec::new();
    let idle_threshold = Duration::from_millis(config.silence_ms as u64 + 50);
    let mut last_audio_at = Instant::now();

    tracing::info!(
        "VAD: initialized (capture={sample_rate}Hz, vad={vad_rate}Hz, \
         pre_resample={needs_pre_resample}, post_resample={needs_post_resample}, \
         frame={vad_frame_size})"
    );

    loop {
        if cancel.is_cancelled() {
            break;
        }

        let raw_audio = match receive_audio(
            &audio_rx,
            &mut last_audio_at,
            &mut first_audio_logged,
            idle_threshold,
            &mut state_machine,
            &metrics,
            &chunk_tx,
        ) {
            Some(data) => data,
            None => continue,
        };

        let vad_samples = match pre_resample_if_needed(&mut pre_accumulator, raw_audio) {
            Some(samples) => samples,
            None => continue,
        };

        sample_buf.extend_from_slice(&vad_samples);

        process_vad_frames(
            &mut sample_buf,
            vad_frame_size,
            &mut state_machine,
            &mut vad,
            &metrics,
            &chunk_tx,
            &mut first_voice_logged,
        );
    }

    if let Some(chunk) = state_machine.flush_speech() {
        emit_chunk(chunk, "flushing final chunk", &metrics, &chunk_tx);
    }
}

/// Receive audio from the capture thread, handling timeouts and idle flushing.
///
/// Returns `Some(data)` when audio is received, `None` on timeout (caller should
/// `continue`). Breaks the caller's loop by returning `None` when disconnected
/// — but the caller also checks `cancel.is_cancelled()`.
fn receive_audio(
    audio_rx: &mpsc::Receiver<Vec<f32>>,
    last_audio_at: &mut Instant,
    first_audio_logged: &mut bool,
    idle_threshold: Duration,
    state_machine: &mut VadStateMachine,
    metrics: &Arc<PipelineMetrics>,
    chunk_tx: &crossbeam_channel::Sender<ChunkEnvelope>,
) -> Option<Vec<f32>> {
    match audio_rx.recv_timeout(std::time::Duration::from_millis(20)) {
        Ok(data) => {
            *last_audio_at = Instant::now();
            if !*first_audio_logged {
                *first_audio_logged = true;
                tracing::info!("VAD: first audio received, {} samples", data.len());
            }
            Some(data)
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            handle_idle_timeout(
                last_audio_at,
                idle_threshold,
                state_machine,
                metrics,
                chunk_tx,
            );
            None
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            tracing::info!("VAD: audio channel closed, exiting");
            None
        }
    }
}

/// Flush buffered speech when no audio has arrived for longer than the idle threshold.
fn handle_idle_timeout(
    last_audio_at: &Instant,
    idle_threshold: Duration,
    state_machine: &mut VadStateMachine,
    metrics: &Arc<PipelineMetrics>,
    chunk_tx: &crossbeam_channel::Sender<ChunkEnvelope>,
) {
    if last_audio_at.elapsed() >= idle_threshold
        && let Some(chunk) = state_machine.flush_speech()
    {
        emit_chunk(chunk, "emitting chunk on idle timeout", metrics, chunk_tx);
    }
}

/// Apply pre-resampling when the capture rate is unsupported by webrtc-vad.
fn pre_resample_if_needed(
    accumulator: &mut Option<ResampleAccumulator>,
    raw_audio: Vec<f32>,
) -> Option<Vec<f32>> {
    match accumulator {
        Some(acc) => {
            let out = acc.push(&raw_audio);
            if out.is_empty() { None } else { Some(out) }
        }
        None => Some(raw_audio),
    }
}

/// Drain complete VAD frames from the sample buffer and emit speech chunks.
fn process_vad_frames(
    sample_buf: &mut Vec<f32>,
    vad_frame_size: usize,
    state_machine: &mut VadStateMachine,
    vad: &mut webrtc_vad::Vad,
    metrics: &Arc<PipelineMetrics>,
    chunk_tx: &crossbeam_channel::Sender<ChunkEnvelope>,
    first_voice_logged: &mut bool,
) {
    while sample_buf.len() >= vad_frame_size {
        let frame: Vec<f32> = sample_buf.drain(..vad_frame_size).collect();
        let frame_i16 = state_machine.convert_frame_to_i16(&frame);
        let is_voice = vad.is_voice_segment(frame_i16).unwrap_or(false);

        if is_voice && !*first_voice_logged {
            *first_voice_logged = true;
            tracing::info!("VAD: first voice detected");
        }
        if let Some(chunk) = state_machine.process_frame(&frame, is_voice) {
            emit_chunk(chunk, "emitting chunk", metrics, chunk_tx);
        }
    }
}

/// Log and send a completed speech chunk to the inference channel.
fn emit_chunk(
    chunk: Vec<f32>,
    label: &str,
    metrics: &Arc<PipelineMetrics>,
    chunk_tx: &crossbeam_channel::Sender<ChunkEnvelope>,
) {
    let len = chunk.len();
    let secs = len as f64 / 16000.0;
    let seq = metrics.next_seq();
    tracing::info!("VAD: {label} seq={seq}, {len} samples ({secs:.1}s)");
    let now = Instant::now();
    try_send_chunk(
        chunk_tx,
        ChunkEnvelope {
            samples: chunk,
            enqueued_at: now,
            silence_detected_at: now,
            seq,
        },
        metrics,
    );
}

fn try_send_chunk(
    tx: &crossbeam_channel::Sender<ChunkEnvelope>,
    envelope: ChunkEnvelope,
    metrics: &PipelineMetrics,
) {
    match tx.try_send(envelope) {
        Ok(()) => {}
        Err(TrySendError::Full(_)) => {
            metrics.increment_drops();
            let drops = metrics.drop_count();
            tracing::warn!("VAD chunk channel full, dropping chunk (total drops: {drops})");
        }
        Err(TrySendError::Disconnected(_)) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_short_chunks() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.01,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);
        let speech_frame = vec![0.5; 320];
        for _ in 0..10 {
            sm.process_frame(&speech_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_none(), "short chunk should be dropped");
    }

    #[test]
    fn emits_valid_chunk() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.001,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);
        let speech_frame = vec![0.1; 320];
        for _ in 0..100 {
            sm.process_frame(&speech_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_some(), "valid chunk should be emitted");
    }

    #[test]
    fn drops_low_energy() {
        let config = VadConfig {
            silence_ms: 700,
            energy_threshold: 0.5,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);
        let quiet_frame = vec![0.001; 320];
        for _ in 0..100 {
            sm.process_frame(&quiet_frame, true);
        }
        let silent_frame = vec![0.0; 320];
        let silence_frames = (16000.0 * 0.7 / 320.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }
        assert!(result.is_none(), "low energy chunk should be dropped");
    }

    #[test]
    fn vad_config_defaults() {
        let config = VadConfig::default();
        assert_eq!(config.silence_ms, 300);
        assert!((config.energy_threshold - 0.01).abs() < f32::EPSILON);
        assert_eq!(config.max_chunk_ms, Some(8000));
    }

    #[test]
    fn max_chunk_ms_forces_emission() {
        // 400ms = 6400 samples at 16kHz, exceeds min_chunk_samples (4800)
        let config = VadConfig {
            silence_ms: 2000,
            energy_threshold: 0.001,
            max_chunk_ms: Some(400),
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);

        let frame: Vec<f32> = vec![0.5; 320]; // 20ms frame

        // 6400 samples / 320 per frame = 20 frames to reach the limit
        for i in 0..25 {
            let result = sm.process_frame(&frame, true);
            if i >= 19 {
                if let Some(chunk) = result {
                    assert!(chunk.len() >= 6400);
                    return;
                }
            }
        }
        panic!("Expected forced emission but none occurred");
    }

    #[test]
    fn max_chunk_ms_none_allows_unbounded() {
        let config = VadConfig {
            silence_ms: 400,
            energy_threshold: 0.001,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);

        let frame: Vec<f32> = vec![0.5; 320];
        for _ in 0..100 {
            assert!(sm.process_frame(&frame, true).is_none());
        }
    }

    #[test]
    fn accumulator_exact_chunk() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        let input = vec![0.0f32; n];
        let output = acc.push(&input);
        assert!(!output.is_empty(), "exact chunk should produce output");
        assert!(acc.buf.is_empty(), "no remainder after exact chunk");
    }

    #[test]
    fn accumulator_undersized_no_output() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        let input = vec![0.0f32; n / 2];
        let output = acc.push(&input);
        assert!(
            output.is_empty(),
            "undersized input should produce no output"
        );
        assert_eq!(acc.buf.len(), n / 2, "remainder should be buffered");
    }

    #[test]
    fn accumulator_multi_push() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        let half = vec![0.0f32; n / 2];
        let out1 = acc.push(&half);
        assert!(out1.is_empty());
        let out2 = acc.push(&half);
        assert!(
            !out2.is_empty(),
            "should produce output after accumulating full chunk"
        );
        let out3 = acc.push(&half);
        assert!(out3.is_empty());
        assert_eq!(acc.buf.len(), n / 2);
    }

    #[test]
    fn chunk_envelope_fields() {
        let envelope = ChunkEnvelope {
            samples: vec![0.1, 0.2, 0.3],
            enqueued_at: Instant::now(),
            silence_detected_at: Instant::now(),
            seq: 42,
        };
        assert_eq!(envelope.samples.len(), 3);
        assert_eq!(envelope.seq, 42);
    }

    #[test]
    fn pipeline_metrics_drop_count() {
        let metrics = PipelineMetrics::new();
        assert_eq!(metrics.drop_count(), 0);
        metrics.increment_drops();
        metrics.increment_drops();
        assert_eq!(metrics.drop_count(), 2);
    }

    #[test]
    fn pipeline_metrics_next_seq() {
        let metrics = PipelineMetrics::new();
        assert_eq!(metrics.next_seq(), 0);
        assert_eq!(metrics.next_seq(), 1);
        assert_eq!(metrics.next_seq(), 2);
    }

    #[test]
    fn accumulator_oversized_concatenates() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        let input = vec![0.0f32; n * 2 + n / 2];
        let output = acc.push(&input);
        assert!(!output.is_empty(), "oversized input should produce output");
        assert_eq!(acc.buf.len(), n / 2, "remainder after 2 full drains");
        assert!(
            output.len() > n / 3,
            "output should contain results from multiple process() calls"
        );
    }

    #[test]
    fn resample_batch_produces_16k_output() {
        let input = vec![0.0f32; 48000]; // 1s at 48kHz
        let output = resample_batch(48000, &input);
        let tolerance = 200;
        assert!(
            (output.len() as i64 - 16000).unsigned_abs() < tolerance,
            "expected ~16000 samples, got {}",
            output.len()
        );
    }

    #[test]
    fn select_vad_rate_16k_no_resample() {
        let (rate, hz, pre) = select_vad_rate(16000, false);
        assert!(matches!(rate, webrtc_vad::SampleRate::Rate16kHz));
        assert_eq!(hz, 16000);
        assert!(!pre);
    }

    #[test]
    fn select_vad_rate_48k_native() {
        let (rate, hz, pre) = select_vad_rate(48000, true);
        assert!(matches!(rate, webrtc_vad::SampleRate::Rate48kHz));
        assert_eq!(hz, 48000);
        assert!(!pre);
    }

    #[test]
    fn select_vad_rate_44100_pre_resample() {
        let (rate, hz, pre) = select_vad_rate(44100, true);
        assert!(matches!(rate, webrtc_vad::SampleRate::Rate16kHz));
        assert_eq!(hz, 16000);
        assert!(pre);
    }

    #[test]
    fn post_resample_finalize_chunk() {
        let config = VadConfig {
            silence_ms: 300,
            energy_threshold: 0.001,
            max_chunk_ms: None,
        };
        // Simulate 48kHz VAD with post-resampling
        let mut sm = VadStateMachine::new(&config, 48000, true);

        // Feed 1s of speech at 48kHz in 480-sample frames (10ms)
        let frame = vec![0.1f32; 480];
        for _ in 0..100 {
            sm.process_frame(&frame, true);
        }

        // Feed silence to trigger emission: 300ms at 48kHz = 14400 samples
        let silent_frame = vec![0.0f32; 480];
        let silence_frames = (48000.0 * 0.3 / 480.0) as usize + 1;
        let mut result = None;
        for _ in 0..silence_frames {
            if let Some(chunk) = sm.process_frame(&silent_frame, false) {
                result = Some(chunk);
            }
        }

        let chunk = result.expect("should emit a resampled chunk");
        // Input was 48000 samples at 48kHz (1s), output should be ~16000 at 16kHz
        let tolerance = 200;
        assert!(
            (chunk.len() as i64 - 16000).unsigned_abs() < tolerance,
            "expected ~16000 samples, got {}",
            chunk.len()
        );
    }

    #[test]
    fn accumulator_flush_remaining() {
        let mut acc = ResampleAccumulator::new(48000);
        let n = acc.resampler.input_frames_next();
        let input = vec![0.0f32; n + n / 2];
        let output = acc.push(&input);
        assert!(!output.is_empty());
        assert_eq!(acc.buf.len(), n / 2);
        let flushed = acc.flush();
        assert!(
            !flushed.is_empty(),
            "flush should produce output from remaining samples"
        );
        assert!(acc.buf.is_empty(), "buffer should be empty after flush");
    }

    #[test]
    fn flush_speech_emits_buffered_chunk() {
        let config = VadConfig {
            silence_ms: 300,
            energy_threshold: 0.001,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);

        // Feed 0.5s of speech
        let frame = vec![0.5f32; 320]; // 20ms
        for _ in 0..25 {
            sm.process_frame(&frame, true);
        }

        // flush_speech should emit the chunk without silence detection
        let chunk = sm.flush_speech();
        assert!(chunk.is_some(), "flush_speech should emit buffered speech");
    }

    #[test]
    fn flush_speech_noop_when_not_in_speech() {
        let config = VadConfig::default();
        let mut sm = VadStateMachine::new(&config, 16000, false);
        assert!(sm.flush_speech().is_none());
    }

    #[test]
    fn flush_speech_respects_min_length() {
        let config = VadConfig {
            silence_ms: 300,
            energy_threshold: 0.001,
            max_chunk_ms: None,
        };
        let mut sm = VadStateMachine::new(&config, 16000, false);

        // Feed only 0.1s of speech (below min_chunk_samples of 0.3s)
        let frame = vec![0.5f32; 320];
        for _ in 0..5 {
            sm.process_frame(&frame, true);
        }

        let chunk = sm.flush_speech();
        assert!(
            chunk.is_none(),
            "flush_speech should respect min_chunk_samples"
        );
    }
}
