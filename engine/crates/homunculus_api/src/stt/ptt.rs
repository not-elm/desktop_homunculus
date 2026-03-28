//! PTT (Push-to-Talk) session management.
//!
//! Provides [`PttSessionRegistry`] as a Bevy [`Resource`] for managing
//! active PTT recording sessions. Designed for single-session operation:
//! at most one session is active at any time.

use std::collections::HashMap;
use std::time::Instant;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use homunculus_microphone::SttModelSize;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// Maximum allowed timeout in seconds.
pub const MAX_TIMEOUT_SECS: u64 = 300;

/// Default timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 60;

fn default_language() -> String {
    "auto".to_string()
}

fn default_timeout_secs() -> u64 {
    DEFAULT_TIMEOUT_SECS
}

/// Request options for starting a PTT session.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PttStartOptions {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub model_size: SttModelSize,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

/// Response for a successful PTT start.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PttStartResponse {
    pub session_id: Uuid,
}

/// An active PTT recording session.
pub struct PttSession {
    pub cancel_token: CancellationToken,
    pub buffer_task: Option<JoinHandle<Vec<f32>>>,
    pub timeout_task: JoinHandle<()>,
    pub sample_rate: u32,
    pub needs_resample: bool,
    pub language: String,
    pub model_size: SttModelSize,
    pub started_at: Instant,
}

impl Drop for PttSession {
    fn drop(&mut self) {
        self.cancel_token.cancel();
        if let Some(task) = &self.buffer_task {
            task.abort();
        }
        self.timeout_task.abort();
    }
}

/// Result of attempting to remove a session from the registry.
pub enum SessionRemoveResult {
    /// Session found and removed.
    Found(PttSession),
    /// Session was already expired by timeout.
    Expired,
    /// Session ID is unknown.
    NotFound,
}

/// Bevy Resource managing active PTT sessions.
///
/// Enforces a single-session constraint: at most one session is active.
/// New sessions automatically cancel existing ones.
#[derive(Resource, Default)]
pub struct PttSessionRegistry {
    active: Option<(Uuid, PttSession)>,
    expired: HashMap<Uuid, Instant>,
}

impl PttSessionRegistry {
    /// Insert a new session, cancelling any existing one.
    ///
    /// Returns the UUID of the cancelled session, if any.
    pub fn insert(&mut self, id: Uuid, session: PttSession) -> Option<Uuid> {
        self.cleanup_expired();
        let cancelled = self.active.take().map(|(old_id, _session)| old_id);
        self.active = Some((id, session));
        cancelled
    }

    /// Remove a session by ID. Returns [`SessionRemoveResult`] indicating
    /// whether the session was found, expired, or unknown.
    pub fn remove(&mut self, id: &Uuid) -> SessionRemoveResult {
        if let Some((active_id, _)) = &self.active
            && active_id == id
        {
            let (_, session) = self.active.take().unwrap();
            return SessionRemoveResult::Found(session);
        }
        if self.expired.contains_key(id) {
            self.expired.remove(id);
            return SessionRemoveResult::Expired;
        }
        SessionRemoveResult::NotFound
    }

    /// Record a session as expired (called by the timeout task).
    pub fn mark_expired(&mut self, id: &Uuid) {
        if let Some((active_id, _)) = &self.active
            && active_id == id
        {
            self.active.take();
            self.expired.insert(*id, Instant::now());
        }
    }

    fn cleanup_expired(&mut self) {
        let ttl = std::time::Duration::from_secs(120);
        self.expired.retain(|_, instant| instant.elapsed() < ttl);
    }
}

/// Bevy Plugin that initializes the PTT session registry.
pub struct SttPttPlugin;

impl Plugin for SttPttPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PttSessionRegistry>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_session() -> PttSession {
        let cancel = CancellationToken::new();
        let token = cancel.clone();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let buffer_task = rt.spawn(async move {
            token.cancelled().await;
            vec![0.0f32; 100]
        });
        let token2 = cancel.clone();
        let timeout_task = rt.spawn(async move {
            token2.cancelled().await;
        });
        std::mem::forget(rt);
        PttSession {
            cancel_token: cancel,
            buffer_task: Some(buffer_task),
            timeout_task,
            sample_rate: 16000,
            needs_resample: false,
            language: "auto".to_string(),
            model_size: SttModelSize::default(),
            started_at: Instant::now(),
        }
    }

    #[test]
    fn insert_returns_none_when_empty() {
        let mut registry = PttSessionRegistry::default();
        let id = Uuid::new_v4();
        let cancelled = registry.insert(id, dummy_session());
        assert!(cancelled.is_none());
        assert!(registry.active.is_some());
    }

    #[test]
    fn insert_cancels_existing_session() {
        let mut registry = PttSessionRegistry::default();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        registry.insert(id1, dummy_session());
        let cancelled = registry.insert(id2, dummy_session());
        assert_eq!(cancelled, Some(id1));
    }

    #[test]
    fn remove_returns_found_for_active_session() {
        let mut registry = PttSessionRegistry::default();
        let id = Uuid::new_v4();
        registry.insert(id, dummy_session());
        assert!(matches!(
            registry.remove(&id),
            SessionRemoveResult::Found(_)
        ));
        assert!(registry.active.is_none());
    }

    #[test]
    fn remove_returns_not_found_for_unknown_id() {
        let mut registry = PttSessionRegistry::default();
        let id = Uuid::new_v4();
        assert!(matches!(
            registry.remove(&id),
            SessionRemoveResult::NotFound
        ));
    }

    #[test]
    fn remove_returns_expired_after_mark_expired() {
        let mut registry = PttSessionRegistry::default();
        let id = Uuid::new_v4();
        registry.insert(id, dummy_session());
        registry.mark_expired(&id);
        assert!(registry.active.is_none());
        assert!(matches!(registry.remove(&id), SessionRemoveResult::Expired));
    }

    #[test]
    fn mark_expired_ignores_wrong_id() {
        let mut registry = PttSessionRegistry::default();
        let id = Uuid::new_v4();
        let wrong_id = Uuid::new_v4();
        registry.insert(id, dummy_session());
        registry.mark_expired(&wrong_id);
        assert!(registry.active.is_some());
    }
}
