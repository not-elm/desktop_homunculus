//! # Homunculus Microphone
//!
//! Real-time speech-to-text crate using whisper-rs.
//! Provides stateless recognition via a `POST /stt/recognize` endpoint.

pub mod capture;
pub mod error;
pub mod inference;
pub mod model;
pub mod permissions;
pub mod vad;

pub use capture::{CaptureHandle, get_input_device, spawn_capture_thread};
pub use error::MicrophoneError;
pub use inference::{InferenceConfig, SttResult, whisper_infer};
pub use model::{
    DownloadProgress, SharedSttModelCache, SttModelCache, SttModelSize, load_whisper_context,
};
pub use permissions::ensure_microphone_permission;
pub use vad::{VadConfig, vad_until_speech};
pub use whisper_rs::WhisperContext;
