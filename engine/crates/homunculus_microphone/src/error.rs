pub type MicrophoneResult<T = ()> = Result<T, MicrophoneError>;

/// Top-level error type for the homunculus_microphone crate.
#[derive(Debug, thiserror::Error)]
pub enum MicrophoneError {
    #[error(transparent)]
    Capture(#[from] CaptureError),
    #[error(transparent)]
    Inference(#[from] InferenceError),
    #[error(transparent)]
    Download(#[from] DownloadError),
    #[error(transparent)]
    Permission(#[from] PermissionError),
    #[error(transparent)]
    Pipeline(#[from] PipelineError),
}

/// Audio capture errors (cpal device/stream).
#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("No microphone device found")]
    NoMicrophone,
    #[error("No supported audio config: {0}")]
    NoSupportedConfig(String),
    #[error("Failed to spawn capture thread: {0}")]
    ThreadSpawn(String),
}

/// Whisper inference errors.
#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("Failed to create whisper state: {0}")]
    CreateState(String),
    #[error("Whisper inference failed: {0}")]
    Full(String),
}

/// Model download errors.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("HTTP request failed: {0}")]
    Request(reqwest::Error),
    #[error("HTTP status {0}")]
    HttpStatus(u16),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Download cancelled")]
    Cancelled,
}

/// Microphone permission errors.
#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("Microphone permission denied")]
    Denied,
    #[error("Microphone permission check failed: {0}")]
    Unknown(String),
}

/// Pipeline orchestration errors.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Capture error: {0}")]
    Capture(String),
    #[error("VAD error: {0}")]
    Vad(String),
    #[error("VAD task failed: {0}")]
    VadFailed(String),
    #[error("Recognition timed out")]
    Timeout,
    #[error("Recognition cancelled")]
    Cancelled,
}
