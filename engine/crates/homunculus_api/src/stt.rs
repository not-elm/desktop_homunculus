//! STT (Speech-to-Text) API.
//!
//! Manages STT sessions and model downloads using `homunculus_microphone`.

use std::sync::Arc;
use std::time::Instant;

use async_broadcast::Receiver;
use homunculus_microphone::session::{SttEvent, SttSession, SttStartOptions, SttState};
use homunculus_microphone::{
    SharedSttModelCache, SharedSttSession, SttModelSize, WhisperContext, get_input_device,
    load_whisper_context,
    model::{
        download_model as mic_download_model, is_model_available, list_available_models, model_path,
    },
    permissions::ensure_microphone_permission,
    pipeline::spawn_pipeline,
};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// STT API error types with HTTP status code mappings.
#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("Session already active")]
    SessionAlreadyActive,
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
    #[error("Invalid language: {0}")]
    InvalidLanguage(String),
    #[error("Invalid model size")]
    InvalidModelSize,
}

/// Whisper-supported language codes (ISO 639-1) plus "auto" for auto-detection.
const WHISPER_SUPPORTED_LANGUAGES: &[&str] = &[
    "auto", "en", "zh", "de", "es", "ru", "ko", "fr", "ja", "pt", "tr", "pl", "ca", "nl", "ar",
    "sv", "it", "id", "hi", "fi", "vi", "he", "uk", "el", "ms", "cs", "ro", "da", "hu", "ta",
    "no", "th", "ur", "hr", "bg", "lt", "la", "mi", "ml", "cy", "sk", "te", "fa", "lv", "bn",
    "sr", "az", "sl", "kn", "et", "mk", "br", "eu", "is", "hy", "ne", "mn", "bs", "kk", "sq",
    "sw", "gl", "mr", "pa", "si", "km", "sn", "yo", "so", "af", "oc", "ka", "be", "tg", "sd",
    "gu", "am", "yi", "lo", "uz", "fo", "ht", "ps", "tk", "nn", "mt", "sa", "lb", "my", "bo",
    "tl", "mg", "as", "tt", "haw", "ln", "ha", "ba", "jw", "su", "yue",
];

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

/// STT API resource — manages STT sessions and model downloads.
#[derive(Clone)]
pub struct SttApi {
    session: SharedSttSession,
    model_cache: SharedSttModelCache,
}

impl SttApi {
    pub fn new(session: SharedSttSession, model_cache: SharedSttModelCache) -> Self {
        Self {
            session,
            model_cache,
        }
    }

    /// Start an STT session.
    pub async fn start(&self, options: SttStartOptions) -> Result<SttState, SttError> {
        let size = options.model_size;
        let language = options.language.clone();

        if !WHISPER_SUPPORTED_LANGUAGES.contains(&language.as_str()) {
            return Err(SttError::InvalidLanguage(language));
        }

        self.try_begin_loading(size, &language).await?;

        let ctx = self.load_or_get_context(size).await?;

        let mut session = self.session.0.lock().await;
        if !matches!(session.state, SttState::Loading { .. }) {
            return Ok(session.state.clone());
        }

        self.launch_pipeline(&mut session, ctx, size, language)
            .await
    }

    /// Stop the current STT session. Idempotent.
    pub async fn stop(&self) -> SttState {
        let mut session = self.session.0.lock().await;
        session.stop();
        session.state.clone()
    }

    /// Get the current session status.
    pub async fn status(&self) -> SttState {
        let session = self.session.0.lock().await;
        session.state.clone()
    }

    /// Get a new SSE event receiver.
    pub async fn new_event_receiver(&self) -> Receiver<SttEvent> {
        let session = self.session.0.lock().await;
        session.new_event_receiver()
    }

    /// Get the current state for late-join sync.
    pub async fn current_state(&self) -> SttState {
        let session = self.session.0.lock().await;
        session.state.clone()
    }

    /// Atomically get current state and event receiver under one lock.
    /// Eliminates TOCTOU race between separate `current_state()` + `new_event_receiver()` calls.
    pub async fn subscribe(&self) -> (SttState, Receiver<SttEvent>) {
        let session = self.session.0.lock().await;
        let state = session.state.clone();
        let rx = session.new_event_receiver();
        (state, rx)
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

    /// Atomically check no active session, verify model availability, and transition to Loading.
    /// Holds a single lock across all three steps to prevent TOCTOU races.
    async fn try_begin_loading(&self, size: SttModelSize, language: &str) -> Result<(), SttError> {
        let mut session = self.session.0.lock().await;
        match &session.state {
            SttState::Loading { .. } | SttState::Listening { .. } => {
                return Err(SttError::SessionAlreadyActive);
            }
            _ => {}
        }
        if !is_model_available(size) {
            return Err(SttError::ModelNotAvailable(format!(
                "Model '{}' is not downloaded. Use POST /stt/models/download first.",
                size.as_str()
            )));
        }
        session.language = language.to_string();
        session.model_size = size;
        session.transition(SttState::Loading {
            language: language.to_string(),
            model_size: size,
        });
        Ok(())
    }

    async fn launch_pipeline(
        &self,
        session: &mut SttSession,
        ctx: Arc<WhisperContext>,
        size: SttModelSize,
        language: String,
    ) -> Result<SttState, SttError> {
        ensure_microphone_permission()
            .await
            .map_err(|_| SttError::MicrophonePermissionDenied)?;

        let device = get_input_device().map_err(|_| SttError::NoMicrophone)?;

        let cancel = CancellationToken::new();
        let started_at = Instant::now();
        session.cancel = Some(cancel.clone());
        session.started_at = Some(started_at);

        spawn_pipeline(
            device,
            ctx,
            language.clone(),
            cancel,
            session.event_tx.clone(),
            self.session.clone(),
            started_at,
        )
        .map_err(|e| SttError::PipelineFailed(e.to_string()))?;

        session.transition(SttState::Listening {
            language,
            model_size: size,
        });

        Ok(session.state.clone())
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
        self.mark_downloading(size).await;
        let result = mic_download_model(size, &CancellationToken::new()).await;
        self.unmark_downloading(size).await;

        result
            .map(|()| downloaded_response(size))
            .map_err(|e| SttError::DownloadFailed(e.to_string()))
    }

    async fn mark_downloading(&self, size: SttModelSize) {
        let mut cache = self.model_cache.0.lock().await;
        cache.mark_downloading(size);
    }

    async fn unmark_downloading(&self, size: SttModelSize) {
        let mut cache = self.model_cache.0.lock().await;
        cache.unmark_downloading(size);
    }
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
