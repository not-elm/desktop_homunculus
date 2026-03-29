//! STT (Speech-to-Text) API.
//!
//! Stateless speech recognition and model downloads using `homunculus_microphone`.

pub mod ptt;

pub use ptt::{PttSessionRegistry, PttStartOptions, PttStartResponse, SttPttPlugin};

use std::sync::Arc;
use std::time::Instant;

use crate::prelude::ApiReactor;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_microphone::{
    DownloadProgress, InferenceConfig, SharedSttModelCache, SttModelSize, SttResult, VadConfig,
    WhisperContext, get_input_device, load_whisper_context,
    model::{
        download_model as mic_download_model, is_model_available, list_available_models, model_path,
    },
    permissions::ensure_microphone_permission,
    spawn_capture_thread, vad_until_speech, whisper_infer,
};
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// STT API error types with HTTP status code mappings.
#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("Model not available: {0}")]
    ModelNotAvailable(String),
    #[error("Model load failed: {0}")]
    ModelLoadFailed(String),
    #[error("Pipeline failed: {0}")]
    PipelineFailed(String),
    #[error("No microphone device found")]
    NoMicrophone,
    #[error("Microphone permission denied")]
    MicrophonePermissionDenied,
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Download cancelled")]
    DownloadCancelled,
    #[error("Invalid language: {0}")]
    InvalidLanguage(String),
    #[error("Invalid model size")]
    InvalidModelSize,
    #[error("Audio below energy threshold")]
    BelowEnergyThreshold,
    #[error("PTT session not found: {0}")]
    SessionNotFound(String),
    #[error("PTT session expired: {0}")]
    SessionExpired(String),
}

/// Whisper-supported language codes (ISO 639-1) plus "auto" for auto-detection.
const WHISPER_SUPPORTED_LANGUAGES: &[&str] = &[
    "auto", "en", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca", "nl", "ar",
    "sv", "it", "id", "hi", "fi", "vi", "he", "uk", "el", "ms", "cs", "ro", "da", "hu", "ta", "no",
    "th", "ur", "hr", "bg", "lt", "la", "mi", "ml", "cy", "sk", "te", "fa", "lv", "bn", "sr", "az",
    "sl", "kn", "et", "mk", "br", "eu", "is", "hy", "ne", "mn", "bs", "kk", "sq", "sw", "gl", "mr",
    "pa", "si", "km", "sn", "yo", "so", "af", "oc", "ka", "be", "tg", "sd", "gu", "am", "yi", "lo",
    "uz", "fo", "ht", "ps", "tk", "nn", "mt", "sa", "lb", "my", "bo", "tl", "mg", "as", "tt",
    "haw", "ln", "ha", "ba", "jw", "su", "yue",
];

/// Request options for a stateless recognition call.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct RecognizeOptions {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub model_size: SttModelSize,
}

fn default_language() -> String {
    "auto".to_string()
}

/// Response for model download endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ModelDownloadResponse {
    pub model_size: SttModelSize,
    pub status: DownloadStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Download status.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum DownloadStatus {
    Downloaded,
    AlreadyExists,
    Downloading,
}

/// Model info for the list endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub model_size: SttModelSize,
    pub size_bytes: u64,
    pub path: String,
}

/// Bevy resource wrapping the parent cancellation token for STT downloads.
///
/// Cancelling this token propagates to all in-progress download child tokens.
/// Used by the `AppExit` shutdown system to cancel downloads without acquiring
/// the `SharedSttModelCache` mutex.
#[derive(Clone)]
pub struct SttShutdownToken(CancellationToken);

impl SttShutdownToken {
    /// Cancel all in-progress downloads by cancelling the parent token.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}

/// Cancels the capture pipeline when dropped (e.g. on client disconnect).
struct PipelineCancelGuard {
    cancel: CancellationToken,
}

impl Drop for PipelineCancelGuard {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

/// STT API resource — stateless speech recognition and model downloads.
#[derive(Clone)]
pub struct SttApi {
    model_cache: SharedSttModelCache,
    shutdown_token: SttShutdownToken,
    reactor: ApiReactor,
}

impl SttApi {
    /// Returns the list of supported language codes.
    pub fn supported_languages() -> &'static [&'static str] {
        WHISPER_SUPPORTED_LANGUAGES
    }

    pub fn new(reactor: ApiReactor) -> Self {
        let parent = CancellationToken::new();
        let shutdown_token = SttShutdownToken(parent.clone());
        Self {
            model_cache: SharedSttModelCache::new(parent),
            shutdown_token,
            reactor,
        }
    }

    /// Returns a clone of the shutdown token for use as a Bevy resource.
    pub fn shutdown_token(&self) -> SttShutdownToken {
        self.shutdown_token.clone()
    }

    /// Perform a single stateless recognition: capture audio, detect speech, infer text.
    pub async fn recognize(&self, options: RecognizeOptions) -> Result<SttResult, SttError> {
        let language = validate_language(options.language)?;
        let ctx = self.load_or_get_context(options.model_size).await?;
        ensure_microphone_access().await?;

        let cancel = CancellationToken::new();
        let _guard = PipelineCancelGuard {
            cancel: cancel.clone(),
        };
        let started_at = Instant::now();
        let (vad_config, inference_config) = load_recognition_configs();

        let capture = start_capture(cancel.clone())?;

        let chunk = vad_until_speech(
            capture.audio_rx,
            capture.sample_rate,
            capture.needs_resample,
            cancel.clone(),
            vad_config,
        )
        .await
        .map_err(|e| SttError::PipelineFailed(e.to_string()))?;

        cancel.cancel();

        run_whisper_inference(ctx, chunk, language, started_at, inference_config).await
    }

    /// Start a PTT recording session.
    ///
    /// Validates options, loads the Whisper model, ensures microphone access,
    /// spawns capture and buffer tasks, and registers the session.
    pub async fn start_ptt(
        &self,
        options: ptt::PttStartOptions,
    ) -> Result<ptt::PttStartResponse, SttError> {
        let language = validate_language(options.language)?;
        let timeout_secs = options.timeout_secs.min(ptt::MAX_TIMEOUT_SECS);
        let model_size = options.model_size;

        let _ctx = self.load_or_get_context(model_size).await?;
        ensure_microphone_access().await?;

        let cancel = CancellationToken::new();
        let capture = start_capture(cancel.clone())?;
        let started_at = std::time::Instant::now();

        let buffer_task = spawn_buffer_task(capture.audio_rx, cancel.clone());

        let session_id = Uuid::new_v4();
        let timeout_cancel = cancel.clone();
        let timeout_reactor = self.reactor.clone();
        let timeout_id = session_id;
        let timeout_task = tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(std::time::Duration::from_secs(timeout_secs)) => {
                    timeout_cancel.cancel();
                    let _ = timeout_reactor
                        .schedule(move |task| async move {
                            task.will(
                                Update,
                                once::run(mark_session_expired).with(timeout_id),
                            )
                            .await;
                        })
                        .await;
                }
                _ = timeout_cancel.cancelled() => {}
            }
        });

        let session = ptt::PttSession {
            cancel_token: cancel,
            buffer_task: Some(buffer_task),
            timeout_task,
            sample_rate: capture.sample_rate,
            needs_resample: capture.needs_resample,
            language,
            model_size,
            started_at,
        };

        self.reactor
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(insert_session).with((session_id, session)),
                )
                .await;
            })
            .await
            .map_err(|e| SttError::PipelineFailed(e.to_string()))?;

        Ok(ptt::PttStartResponse { session_id })
    }

    /// Stop a PTT session and return the recognition result.
    ///
    /// Phase 1 (inside schedule, World lock): remove session from registry.
    /// Phase 2 (outside schedule, no lock): await buffer, run inference.
    pub async fn stop_ptt(
        &self,
        session_id: Uuid,
    ) -> Result<homunculus_microphone::SttResult, SttError> {
        let mut session: ptt::PttSession = self
            .reactor
            .schedule(move |task| async move {
                task.will(Update, once::run(remove_session).with(session_id))
                    .await
            })
            .await
            .map_err(|e| SttError::PipelineFailed(e.to_string()))?
            .map_err(|e| match e {
                RemoveSessionError::NotFound => SttError::SessionNotFound(session_id.to_string()),
                RemoveSessionError::Expired => SttError::SessionExpired(session_id.to_string()),
            })?;

        let started_at = session.started_at;
        let language = session.language.clone();
        let model_size = session.model_size;
        let sample_rate = session.sample_rate;
        let needs_resample = session.needs_resample;

        session.cancel_token.cancel();

        let buffer_task = session
            .buffer_task
            .take()
            .ok_or_else(|| SttError::PipelineFailed("Buffer task already consumed".into()))?;
        let buffer = buffer_task
            .await
            .map_err(|e| SttError::PipelineFailed(format!("Buffer task failed: {e}")))?;

        // Drop session to release microphone (cpal stream drop).
        drop(session);

        if buffer.is_empty() {
            return Ok(homunculus_microphone::SttResult {
                text: String::new(),
                timestamp: started_at.elapsed().as_secs_f64(),
                language,
            });
        }

        let resampled = resample_if_needed(buffer, sample_rate, needs_resample);

        let ctx = self.load_or_get_context(model_size).await?;
        let (_, inference_config) = load_recognition_configs();

        let inference_timeout = std::time::Duration::from_secs(30);
        let result = tokio::time::timeout(
            inference_timeout,
            run_whisper_inference(ctx, resampled, language.clone(), started_at, inference_config),
        )
        .await
        .map_err(|_| SttError::PipelineFailed("Inference timed out".to_string()))?;

        match result {
            Ok(stt_result) => Ok(stt_result),
            Err(SttError::BelowEnergyThreshold) => Ok(empty_result(started_at, language)),
            Err(e) => Err(e),
        }
    }

    /// Download a model. Returns the download status.
    pub async fn download_model(
        &self,
        size: SttModelSize,
    ) -> Result<ModelDownloadResponse, SttError> {
        if is_model_available(size) {
            return Ok(already_exists_response(size));
        }
        if self.is_download_in_progress(size).await {
            return Ok(downloading_response(size));
        }

        self.execute_download(size).await
    }

    /// Cancel a specific model download. Returns `true` if a download was cancelled.
    pub async fn cancel_download(&self, size: SttModelSize) -> bool {
        let cache = self.model_cache.0.lock().await;
        if let Some(token) = cache.get_download_token(size) {
            token.cancel();
            true
        } else {
            false
        }
    }

    /// Cancel all in-progress downloads by cancelling individual child tokens.
    ///
    /// Does NOT cancel the parent token (which would permanently poison future downloads).
    pub async fn cancel_all_downloads(&self) -> usize {
        let cache = self.model_cache.0.lock().await;
        cache.cancel_all_downloads()
    }

    /// List available (downloaded) models.
    pub fn list_models(&self) -> Vec<ModelInfo> {
        list_available_models()
            .into_iter()
            .map(|(size, bytes, _path)| ModelInfo {
                model_size: size,
                size_bytes: bytes,
                path: relative_model_path(size),
            })
            .collect()
    }

    /// Start a streaming model download.
    ///
    /// Returns the progress watch receiver, the download join handle,
    /// and the cancellation token for this download.
    /// The caller is responsible for streaming progress and calling
    /// `finish_download()` when the handle completes.
    pub async fn start_download_stream(
        &self,
        size: SttModelSize,
    ) -> (
        watch::Receiver<DownloadProgress>,
        tokio::task::JoinHandle<Result<(), homunculus_microphone::error::DownloadError>>,
        CancellationToken,
    ) {
        let cancel = self.mark_downloading(size).await;
        let (rx, handle) = mic_download_model(size, &cancel);
        (rx, handle, cancel)
    }

    /// Mark a model download as no longer in progress.
    pub async fn finish_download(&self, size: SttModelSize) {
        self.unmark_downloading(size).await;
    }

    /// Check if a model is already downloaded.
    pub fn is_model_available(&self, size: SttModelSize) -> bool {
        is_model_available(size)
    }

    /// Check if a download is currently in progress for the given model size.
    pub async fn is_downloading(&self, size: SttModelSize) -> bool {
        self.is_download_in_progress(size).await
    }

    async fn load_or_get_context(
        &self,
        size: SttModelSize,
    ) -> Result<Arc<WhisperContext>, SttError> {
        if let Some(cached) = self.get_cached_context(size).await {
            return Ok(cached);
        }

        let ctx = load_context_blocking(size).await?;
        self.cache_context(size, ctx.clone()).await;
        Ok(ctx)
    }

    async fn get_cached_context(&self, size: SttModelSize) -> Option<Arc<WhisperContext>> {
        let cache = self.model_cache.0.lock().await;
        cache.get_context(size)
    }

    async fn cache_context(&self, size: SttModelSize, ctx: Arc<WhisperContext>) {
        let mut cache = self.model_cache.0.lock().await;
        cache.insert_context(size, ctx);
    }

    async fn is_download_in_progress(&self, size: SttModelSize) -> bool {
        let cache = self.model_cache.0.lock().await;
        cache.is_downloading(size)
    }

    async fn execute_download(
        &self,
        size: SttModelSize,
    ) -> Result<ModelDownloadResponse, SttError> {
        let cancel = self.mark_downloading(size).await;
        let tmp_path = model_path(size).with_extension("bin.tmp");
        let (_rx, handle) = mic_download_model(size, &cancel);

        let result = tokio::select! {
            join_result = handle => {
                match join_result {
                    Ok(Ok(())) => Ok(downloaded_response(size)),
                    Ok(Err(homunculus_microphone::error::DownloadError::Cancelled)) => {
                        Err(SttError::DownloadCancelled)
                    }
                    Ok(Err(e)) => Err(SttError::DownloadFailed(e.to_string())),
                    Err(e) => Err(SttError::DownloadFailed(e.to_string())),
                }
            }
            _ = cancel.cancelled() => {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                Err(SttError::DownloadCancelled)
            }
        };

        self.unmark_downloading(size).await;
        result
    }

    async fn mark_downloading(&self, size: SttModelSize) -> CancellationToken {
        let mut cache = self.model_cache.0.lock().await;
        cache
            .mark_downloading(size)
            .unwrap_or_else(|| cache.get_download_token(size).unwrap())
    }

    async fn unmark_downloading(&self, size: SttModelSize) {
        let mut cache = self.model_cache.0.lock().await;
        cache.unmark_downloading(size);
    }
}

/// Spawn a tokio task that collects audio frames into a local buffer.
fn spawn_buffer_task(
    audio_rx: std::sync::mpsc::Receiver<Vec<f32>>,
    cancel: CancellationToken,
) -> JoinHandle<Vec<f32>> {
    tokio::spawn(async move {
        let mut buffer = Vec::new();
        loop {
            if cancel.is_cancelled() {
                break;
            }
            match audio_rx.recv_timeout(std::time::Duration::from_millis(50)) {
                Ok(frames) => buffer.extend_from_slice(&frames),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        buffer
    })
}

/// One-shot Bevy system: insert a PTT session into the registry.
fn insert_session(
    In((id, session)): In<(Uuid, ptt::PttSession)>,
    mut registry: ResMut<ptt::PttSessionRegistry>,
) {
    registry.insert(id, session);
}

/// One-shot Bevy system: mark a PTT session as expired (timeout).
fn mark_session_expired(In(id): In<Uuid>, mut registry: ResMut<ptt::PttSessionRegistry>) {
    registry.mark_expired(&id);
}

enum RemoveSessionError {
    NotFound,
    Expired,
}

/// One-shot Bevy system: remove a PTT session from the registry.
fn remove_session(
    In(id): In<Uuid>,
    mut registry: ResMut<ptt::PttSessionRegistry>,
) -> Result<ptt::PttSession, RemoveSessionError> {
    match registry.remove(&id) {
        ptt::SessionRemoveResult::Found(session) => Ok(session),
        ptt::SessionRemoveResult::Expired => Err(RemoveSessionError::Expired),
        ptt::SessionRemoveResult::NotFound => Err(RemoveSessionError::NotFound),
    }
}

/// Resample audio to 16kHz via linear interpolation if the device rate differs.
fn resample_if_needed(buffer: Vec<f32>, sample_rate: u32, needs_resample: bool) -> Vec<f32> {
    if !needs_resample || sample_rate == 16000 {
        return buffer;
    }
    // Use simple linear interpolation for resampling to 16kHz.
    // The target is always 16000 Hz for Whisper.
    let ratio = 16000.0 / sample_rate as f64;
    let new_len = (buffer.len() as f64 * ratio) as usize;
    let mut resampled = Vec::with_capacity(new_len);
    for i in 0..new_len {
        let src_idx = i as f64 / ratio;
        let idx = src_idx as usize;
        let frac = (src_idx - idx as f64) as f32;
        if idx + 1 < buffer.len() {
            resampled.push(buffer[idx] * (1.0 - frac) + buffer[idx + 1] * frac);
        } else if idx < buffer.len() {
            resampled.push(buffer[idx]);
        }
    }
    resampled
}


fn validate_language(language: String) -> Result<String, SttError> {
    if !WHISPER_SUPPORTED_LANGUAGES.contains(&language.as_str()) {
        return Err(SttError::InvalidLanguage(language));
    }
    Ok(language)
}

/// Verify microphone permission before starting capture.
async fn ensure_microphone_access() -> Result<(), SttError> {
    ensure_microphone_permission()
        .await
        .map_err(|_| SttError::MicrophonePermissionDenied)
}

/// Acquire the default input device and spawn the capture thread.
fn start_capture(
    cancel: CancellationToken,
) -> Result<homunculus_microphone::CaptureHandle, SttError> {
    let device = get_input_device().map_err(|_| SttError::NoMicrophone)?;
    spawn_capture_thread(device, cancel).map_err(|e| SttError::PipelineFailed(e.to_string()))
}

fn load_recognition_configs() -> (VadConfig, InferenceConfig) {
    let config = homunculus_utils::config::HomunculusConfig::load().unwrap_or_default();
    let vad_config = VadConfig::from_stt_config(&config.stt);
    let inference_config = InferenceConfig::from_stt_config(&config.stt);
    (vad_config, inference_config)
}

async fn run_whisper_inference(
    ctx: Arc<WhisperContext>,
    chunk: Vec<f32>,
    language: String,
    started_at: Instant,
    config: InferenceConfig,
) -> Result<SttResult, SttError> {
    tokio::task::spawn_blocking(move || whisper_infer(&ctx, &chunk, &language, started_at, config))
        .await
        .map_err(|e| SttError::PipelineFailed(format!("Inference task panicked: {e}")))?
        .map_err(|e| match e {
            homunculus_microphone::error::InferenceError::BelowEnergyThreshold => {
                SttError::BelowEnergyThreshold
            }
            other => SttError::PipelineFailed(other.to_string()),
        })
}

async fn load_context_blocking(size: SttModelSize) -> Result<Arc<WhisperContext>, SttError> {
    tokio::task::spawn_blocking(move || {
        load_whisper_context(size).map_err(SttError::ModelLoadFailed)
    })
    .await
    .map_err(|e| SttError::ModelLoadFailed(e.to_string()))?
}

fn empty_result(started_at: Instant, language: String) -> SttResult {
    SttResult {
        text: String::new(),
        timestamp: started_at.elapsed().as_secs_f64(),
        language,
    }
}

fn relative_model_path(size: SttModelSize) -> String {
    let path = model_path(size);
    format!(
        "models/{}",
        path.file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default()
    )
}

fn already_exists_response(size: SttModelSize) -> ModelDownloadResponse {
    ModelDownloadResponse {
        model_size: size,
        status: DownloadStatus::AlreadyExists,
        path: Some(relative_model_path(size)),
    }
}

fn downloading_response(size: SttModelSize) -> ModelDownloadResponse {
    ModelDownloadResponse {
        model_size: size,
        status: DownloadStatus::Downloading,
        path: None,
    }
}

fn downloaded_response(size: SttModelSize) -> ModelDownloadResponse {
    ModelDownloadResponse {
        model_size: size,
        status: DownloadStatus::Downloaded,
        path: Some(relative_model_path(size)),
    }
}
