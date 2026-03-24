//! STT (Speech-to-Text) API.
//!
//! Stateless speech recognition and model downloads using `homunculus_microphone`.

use std::sync::Arc;
use std::time::Instant;

use homunculus_microphone::{
    DownloadProgress, InferenceConfig, SharedSttModelCache, SttModelSize, SttResult, VadConfig,
    WhisperContext, get_input_device, load_whisper_context, spawn_capture_thread,
    vad_until_speech, whisper_infer,
    model::{
        download_model as mic_download_model, is_model_available, list_available_models, model_path,
    },
    permissions::ensure_microphone_permission,
};
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

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
}

impl SttApi {
    /// Returns the list of supported language codes.
    pub fn supported_languages() -> &'static [&'static str] {
        WHISPER_SUPPORTED_LANGUAGES
    }

    pub fn new() -> Self {
        let parent = CancellationToken::new();
        let shutdown_token = SttShutdownToken(parent.clone());
        Self {
            model_cache: SharedSttModelCache::new(parent),
            shutdown_token,
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
fn start_capture(cancel: CancellationToken) -> Result<homunculus_microphone::CaptureHandle, SttError> {
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
    tokio::task::spawn_blocking(move || {
        whisper_infer(&ctx, &chunk, &language, started_at, config)
    })
    .await
    .map_err(|e| SttError::PipelineFailed(format!("Inference task panicked: {e}")))?
    .map_err(|e| SttError::PipelineFailed(e.to_string()))
}

async fn load_context_blocking(size: SttModelSize) -> Result<Arc<WhisperContext>, SttError> {
    tokio::task::spawn_blocking(move || {
        load_whisper_context(size).map_err(SttError::ModelLoadFailed)
    })
    .await
    .map_err(|e| SttError::ModelLoadFailed(e.to_string()))?
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
