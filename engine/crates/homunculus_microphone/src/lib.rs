//! # Homunculus Microphone
//!
//! Real-time speech-to-text crate using whisper-rs.
//! Manages a 3-thread pipeline: cpal audio capture -> VAD chunking -> Whisper inference.
//! State is managed via `Arc<tokio::sync::Mutex>` for direct HTTP integration
//! without Bevy's ApiReactor pattern.

pub mod capture;
pub mod inference;
pub mod model;
pub mod permissions;
pub mod pipeline;
pub mod session;
pub mod vad;

pub use capture::get_input_device;
pub use model::{SharedSttModelCache, SttModelCache, SttModelSize};
pub use permissions::ensure_microphone_permission;
pub use pipeline::spawn_pipeline;
pub use session::{SharedSttSession, SttEvent, SttSession, SttStartOptions, SttState};
