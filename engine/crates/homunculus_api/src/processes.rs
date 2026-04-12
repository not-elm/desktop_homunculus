//! Managed processes API.
//!
//! Provides async wrappers for starting, stopping, and listing long-running
//! MOD command processes. Includes an exit detection system that emits
//! `process:exited` signals when processes terminate unexpectedly.

use crate::api;
use crate::error::{ApiError, ApiResult};
use crate::signals::SignalsChannels;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{HomunculusConfig, ModRegistry};
use homunculus_mod::managed_process::{MAX_PROCESSES, ManagedProcess};
use homunculus_utils::runtime::RuntimeResolver;
use homunculus_mod::node_process::NodeProcessHandle;
use serde::{Deserialize, Serialize};
use std::time::Duration;

api!(
    /// Provides access to the managed processes API.
    ProcessesApi
);

/// Request to start a managed process.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct StartProcessRequest {
    /// Full MOD command reference (`mod-name:bin-name`).
    pub command: String,
    /// Arguments forwarded to the process as CLI args.
    #[serde(default)]
    pub args: Vec<String>,
}

/// Response from starting a managed process.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct StartProcessResponse {
    /// Unique handle ID for the started process.
    pub handle_id: String,
}

/// Information about a running managed process.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    /// Unique handle identifier.
    pub handle_id: String,
    /// Full MOD command reference.
    pub command: String,
    /// Arguments passed to the process.
    pub args: Vec<String>,
    /// OS process ID.
    pub pid: u32,
    /// ISO 8601 timestamp of when the process was started.
    pub started_at: String,
}

impl ProcessesApi {
    /// Start a new managed process.
    pub async fn start(&self, req: StartProcessRequest) -> ApiResult<StartProcessResponse> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(start_process).with(req)).await
            })
            .await?
    }

    /// Stop a managed process by handle ID.
    ///
    /// The shutdown is fully non-blocking for the Bevy main thread:
    /// entity lookup and despawn happen in a one-shot system, then the
    /// actual SIGTERM → grace → SIGKILL sequence runs in a tokio blocking task.
    pub async fn stop(&self, handle_id: String) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                // Step 1: Take NodeProcessHandle ownership and despawn entity.
                let mut handle = task
                    .will(Update, once::run(take_and_despawn).with(handle_id))
                    .await?;

                // Step 2: Async shutdown in tokio (does NOT block Bevy).
                task.will(
                    Update,
                    side_effect::tokio::spawn(async move {
                        tokio::task::spawn_blocking(move || {
                            handle.shutdown(Duration::from_secs(2));
                        })
                        .await
                        .ok();
                    }),
                )
                .await;

                Ok(())
            })
            .await?
    }

    /// List all running managed processes.
    pub async fn list(&self) -> ApiResult<Vec<ProcessInfo>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_processes)).await })
            .await
    }
}

fn start_process(
    In(req): In<StartProcessRequest>,
    mut commands: Commands,
    registry: Res<ModRegistry>,
    config: Res<HomunculusConfig>,
    runtime: Res<RuntimeResolver>,
    existing: Query<&ManagedProcess>,
) -> ApiResult<StartProcessResponse> {
    if existing.iter().count() >= MAX_PROCESSES {
        return Err(ApiError::TooManyRequests(format!(
            "Maximum {MAX_PROCESSES} concurrent managed processes"
        )));
    }

    let result = homunculus_mod::managed_process::spawn_managed_process(
        &mut commands,
        &registry,
        &config,
        &runtime,
        &req.command,
        req.args,
    )
    .map_err(ApiError::InvalidInput)?;

    Ok(StartProcessResponse {
        handle_id: result.handle_id,
    })
}

/// Takes [`NodeProcessHandle`] ownership from the entity and despawns it.
///
/// This is an exclusive system (`&mut World`) because `entity_mut().take::<T>()`
/// requires immediate component removal (not deferred via `Commands`).
fn take_and_despawn(In(handle_id): In<String>, world: &mut World) -> ApiResult<NodeProcessHandle> {
    let entity = world
        .query::<(Entity, &ManagedProcess)>()
        .iter(world)
        .find(|(_, m)| m.handle_id == handle_id)
        .map(|(e, _)| e)
        .ok_or(ApiError::EntityNotFound)?;
    let handle = world
        .entity_mut(entity)
        .take::<NodeProcessHandle>()
        .ok_or(ApiError::EntityNotFound)?;
    world.despawn(entity);
    Ok(handle)
}

fn list_processes(query: Query<&ManagedProcess>) -> Vec<ProcessInfo> {
    query
        .iter()
        .map(|m| ProcessInfo {
            handle_id: m.handle_id.clone(),
            command: m.command.clone(),
            args: m.args.clone(),
            pid: m.pid,
            started_at: m.started_at.to_rfc3339(),
        })
        .collect()
}

/// Plugin that registers the exit detection system.
pub(crate) struct ProcessesApiPlugin;

impl Plugin for ProcessesApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_process_exits);
    }
}

/// Polls `try_wait_exited()` on all managed processes each frame.
/// Emits `process:exited` signal and despawns the entity when a process has exited.
fn check_process_exits(
    mut commands: Commands,
    mut query: Query<(Entity, &ManagedProcess, &mut NodeProcessHandle)>,
    mut channels: ResMut<SignalsChannels>,
) {
    for (entity, managed, mut handle) in query.iter_mut() {
        if let Some(status) = handle.try_wait_exited() {
            let exit_code = status.code();
            #[cfg(unix)]
            let signal = {
                use std::os::unix::process::ExitStatusExt;
                status.signal().map(|s| format!("{s}"))
            };
            #[cfg(not(unix))]
            let signal: Option<String> = None;

            let reason = if exit_code == Some(0) {
                "exited"
            } else {
                "crashed"
            };

            let payload = serde_json::json!({
                "handleId": managed.handle_id,
                "command": managed.command,
                "exitCode": exit_code,
                "signal": signal,
                "reason": reason,
            });

            if let Err(e) = channels.send_blocking("process:exited", payload) {
                error!(
                    "Failed to send process:exited signal for {}: {e}",
                    managed.handle_id
                );
            }

            info!(
                "Managed process '{}' {reason} (code: {exit_code:?})",
                managed.handle_id
            );
            commands.entity(entity).despawn();
        }
    }
}
