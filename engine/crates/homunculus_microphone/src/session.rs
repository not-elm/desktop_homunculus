use crate::model::SttModelSize;
use async_broadcast::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio_util::sync::CancellationToken;

/// Shared newtype for HTTP state.
#[derive(Clone)]
pub struct SharedSttSession(pub Arc<tokio::sync::Mutex<SttSession>>);

impl Default for SharedSttSession {
    fn default() -> Self {
        Self(Arc::new(tokio::sync::Mutex::new(SttSession::default())))
    }
}

impl SharedSttSession {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Manages the entire STT session.
pub struct SttSession {
    pub state: SttState,
    pub language: String,
    pub model_size: SttModelSize,
    /// Session start time — used as the timestamp baseline.
    pub started_at: Option<Instant>,
    /// Session cancellation signal.
    pub cancel: Option<CancellationToken>,
    /// SSE broadcast sender.
    pub event_tx: Sender<SttEvent>,
    /// Kept to prevent the channel from closing when there are no SSE clients.
    pub _event_rx: Receiver<SttEvent>,
}

impl Default for SttSession {
    fn default() -> Self {
        let (mut tx, rx) = async_broadcast::broadcast::<SttEvent>(256);
        tx.set_overflow(true);
        Self {
            state: SttState::Idle,
            language: "auto".into(),
            model_size: SttModelSize::default(),
            started_at: None,
            cancel: None,
            event_tx: tx,
            _event_rx: rx,
        }
    }
}

impl SttSession {
    /// Creates a new receiver for SSE clients.
    pub fn new_event_receiver(&self) -> Receiver<SttEvent> {
        self.event_tx.new_receiver()
    }

    /// Transitions the state and notifies SSE clients.
    pub fn transition(&mut self, new_state: SttState) {
        self.state = new_state.clone();
        self.event_tx
            .try_broadcast(SttEvent::Status(new_state))
            .ok();
    }

    /// Stops the session by cancelling the token.
    /// The `Stopped` event is sent when the inference loop exits (to prevent double-firing).
    pub fn stop(&mut self) {
        self.cancel_pipeline();
        self.state = SttState::Idle;
    }

    /// Stops the session due to an error.
    /// The `Error` state persists until explicitly cleared by `stop()` or `start()`.
    pub fn fail(&mut self, error: String, message: String) {
        self.cancel_pipeline();
        self.state = SttState::Error {
            error: error.clone(),
            message: message.clone(),
        };
        self.event_tx
            .try_broadcast(SttEvent::Status(self.state.clone()))
            .ok();
        self.event_tx
            .try_broadcast(SttEvent::SessionError { error, message })
            .ok();
    }

    /// Cancels the running pipeline and clears the start timestamp.
    fn cancel_pipeline(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.cancel();
        }
        self.started_at = None;
    }
}

/// STT session state.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum SttState {
    Idle,
    Loading {
        language: String,
        #[serde(rename = "modelSize")]
        model_size: SttModelSize,
    },
    Listening {
        language: String,
        #[serde(rename = "modelSize")]
        model_size: SttModelSize,
    },
    Error {
        error: String,
        message: String,
    },
}

/// Events sent from the session to SSE clients.
#[derive(Clone, Debug)]
pub enum SttEvent {
    Status(SttState),
    Result {
        text: String,
        timestamp: f64,
        language: String,
    },
    SessionError {
        error: String,
        message: String,
    },
    Stopped,
}

/// Session start options.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SttStartOptions {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub model_size: SttModelSize,
}

impl Default for SttStartOptions {
    fn default() -> Self {
        Self {
            language: default_language(),
            model_size: SttModelSize::default(),
        }
    }
}

fn default_language() -> String {
    "auto".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_session_is_idle() {
        let session = SttSession::default();
        assert!(matches!(session.state, SttState::Idle));
        assert!(session.started_at.is_none());
        assert!(session.cancel.is_none());
    }

    #[test]
    fn transition_updates_state() {
        let mut session = SttSession::default();
        session.transition(SttState::Listening {
            language: "ja".into(),
            model_size: SttModelSize::Small,
        });
        assert!(matches!(session.state, SttState::Listening { .. }));
    }

    #[test]
    fn stop_resets_to_idle() {
        let mut session = SttSession::default();
        session.cancel = Some(CancellationToken::new());
        session.started_at = Some(Instant::now());
        session.transition(SttState::Listening {
            language: "auto".into(),
            model_size: SttModelSize::Small,
        });
        session.stop();
        assert!(matches!(session.state, SttState::Idle));
        assert!(session.started_at.is_none());
        assert!(session.cancel.is_none());
    }

    #[test]
    fn fail_sets_persistent_error_state() {
        let mut session = SttSession::default();
        session.cancel = Some(CancellationToken::new());
        session.started_at = Some(Instant::now());
        session.fail("device_lost".into(), "Microphone disconnected".into());
        if let SttState::Error { error, message } = &session.state {
            assert_eq!(error, "device_lost");
            assert_eq!(message, "Microphone disconnected");
        } else {
            panic!("expected Error state");
        }
        assert!(session.started_at.is_none());
        assert!(session.cancel.is_none());
    }

    #[test]
    fn stop_from_error_resets_to_idle() {
        let mut session = SttSession::default();
        session.fail("device_lost".into(), "test".into());
        assert!(matches!(session.state, SttState::Error { .. }));
        session.stop();
        assert!(matches!(session.state, SttState::Idle));
    }

    #[test]
    fn stt_state_serde_idle() {
        let state = SttState::Idle;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, r#"{"state":"idle"}"#);
    }

    #[test]
    fn stt_state_serde_listening() {
        let state = SttState::Listening {
            language: "ja".into(),
            model_size: SttModelSize::Small,
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains(r#""state":"listening""#));
        assert!(json.contains(r#""language":"ja""#));
        assert!(json.contains(r#""modelSize":"small""#));
    }

    #[test]
    fn stt_state_serde_error() {
        let state = SttState::Error {
            error: "device_lost".into(),
            message: "Microphone disconnected".into(),
        };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains(r#""state":"error""#));
        assert!(json.contains(r#""error":"device_lost""#));
    }

    #[test]
    fn start_options_defaults() {
        let opts = SttStartOptions::default();
        assert_eq!(opts.language, "auto");
        assert_eq!(opts.model_size, SttModelSize::Small);
    }
}
