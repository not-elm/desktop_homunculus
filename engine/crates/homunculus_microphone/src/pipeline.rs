use std::sync::Arc;
use std::time::Instant;
use tokio_util::sync::CancellationToken;
use whisper_rs::WhisperContext;

use crate::capture::{self, CaptureHandle};
use crate::error::PipelineError;
use crate::inference;
use crate::session::{SharedSttSession, SttEvent};
use crate::vad::{self, PipelineMetrics, VadConfig};

/// Spawn the 3-thread pipeline: capture -> VAD -> inference.
pub fn spawn_pipeline(
    device: cpal::Device,
    ctx: Arc<WhisperContext>,
    language: String,
    cancel: CancellationToken,
    event_tx: async_broadcast::Sender<SttEvent>,
    session: SharedSttSession,
    started_at: Instant,
) -> Result<(), PipelineError> {
    let config = homunculus_utils::config::HomunculusConfig::load().unwrap_or_default();
    let vad_config = VadConfig::from_stt_config(&config.stt);
    let no_speech_discard_threshold = config.stt.no_speech_threshold.unwrap_or(0.8);

    let CaptureHandle {
        audio_rx,
        sample_rate,
        needs_resample,
    } = capture::spawn_capture_thread(device, cancel.clone(), session.clone())
        .map_err(|e| PipelineError::Capture(e.to_string()))?;

    let metrics = PipelineMetrics::new();

    let chunk_rx = vad::spawn_vad_thread(
        audio_rx,
        sample_rate,
        needs_resample,
        cancel.clone(),
        vad_config,
        metrics,
    )?;

    inference::spawn_inference_thread(
        ctx,
        chunk_rx,
        language,
        cancel,
        event_tx,
        session,
        started_at,
        no_speech_discard_threshold,
    );

    Ok(())
}
