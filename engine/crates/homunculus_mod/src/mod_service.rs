use crate::node_process::{NodeAvailable, NodeProcessHandle};
use bevy::prelude::*;
use homunculus_core::prelude::{McpDeregisterSender, SharedRpcRegistry};
use homunculus_utils::process::CommandNoWindow;
use homunculus_utils::runtime::RuntimeResolver;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Stdio;

/// A MOD service identified by its absolute filesystem path.
///
/// Services are long-running Node.js child processes that run for the
/// entire app session, declared via the `homunculus.service` field in a MOD's `package.json`.
#[derive(Component)]
pub(crate) struct ModService {
    pub mod_name: String,
    pub script_path: PathBuf,
    pub mods_dir: PathBuf,
}

/// Marker component attached to entities that own a mod service process which
/// was registered with the MCP extension registry.
///
/// The [`watch_mod_service_processes`] system polls these entities and sends a
/// deregister signal via [`McpDeregisterSender`] when the process exits.
#[derive(Component)]
pub struct ModServiceProcess {
    pub mod_slug: String,
}

pub(crate) struct ModServicePlugin;

impl Plugin for ModServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                run_mod_services.run_if(resource_exists::<NodeAvailable>),
                watch_mod_service_processes,
            ),
        );
    }
}

fn run_mod_services(
    mut commands: Commands,
    services: Query<(Entity, &ModService)>,
    rpc_registry: Res<SharedRpcRegistry>,
    runtime: Res<RuntimeResolver>,
) {
    for (entity, service) in services.iter() {
        info!("Starting mod service: {}", service.script_path.display());

        let rpc_port = match allocate_ephemeral_port() {
            Ok(port) => port,
            Err(e) => {
                error!(
                    "Failed to allocate RPC port for mod '{}': {}",
                    service.mod_name, e
                );
                commands.entity(entity).despawn();
                continue;
            }
        };

        pre_register_rpc_port(&rpc_registry, &service.mod_name, rpc_port);

        match launch_mod_service_process(service, rpc_port, &runtime) {
            Ok(child) => {
                append_pid_file(child.id());
                let mod_slug = derive_slug(&service.mod_name);
                commands.spawn((
                    build_process_handle(child, &service.mod_name),
                    ModServiceProcess { mod_slug },
                ));
            }
            Err(e) => {
                error!(
                    "Failed to start mod service {}: {}",
                    service.script_path.display(),
                    e
                );
            }
        }
        commands.entity(entity).despawn();
    }
}

/// Polls all running mod service processes for exit and enqueues MCP deregistrations.
///
/// When a process exits, its slug is sent to the [`McpDeregisterSender`] background
/// task for async removal, and the entity is despawned.
pub(crate) fn watch_mod_service_processes(
    mut commands: Commands,
    mut query: Query<(Entity, &ModServiceProcess, &mut NodeProcessHandle)>,
    deregister: Option<Res<McpDeregisterSender>>,
) {
    for (entity, tag, mut handle) in query.iter_mut() {
        if let Some(status) = handle.try_wait_exited() {
            info!(
                mod_slug = %tag.mod_slug,
                ?status,
                "mod service process exited"
            );
            enqueue_deregister(&deregister, &tag.mod_slug);
            commands.entity(entity).despawn();
        }
    }
}

/// Sends `slug` to the async deregister background task if the resource is present.
fn enqueue_deregister(deregister: &Option<Res<McpDeregisterSender>>, slug: &str) {
    let Some(sender) = deregister else { return };
    if sender.0.send(slug.to_owned()).is_err() {
        warn!(mod_slug = %slug, "MCP deregister channel closed; skipping deregistration");
    }
}

/// Derives a canonical slug from a mod package name.
///
/// Takes the last path component (after the last `/`), lowercases it,
/// and replaces `-` with `_` to match the `^[a-z][a-z0-9_]*$` slug pattern.
///
/// # Example
/// `@hmcs/voicevox` → `voicevox`, `my-mod` → `my_mod`
fn derive_slug(mod_name: &str) -> String {
    let last = mod_name.rsplit('/').next().unwrap_or(mod_name);
    last.replace('-', "_").to_lowercase()
}

/// Append a child PID to `~/.homunculus/mod_pids` so stale processes can be
/// detected and cleaned up on next launch.
pub fn append_pid_file(pid: u32) {
    use std::io::Write;
    let path = homunculus_utils::path::homunculus_dir().join("mod_pids");
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
    {
        let _ = writeln!(f, "{pid}");
    }
}

/// Pre-registers the port in the RPC registry so the HTTP proxy can route
/// requests even before the MOD service calls back to register its methods.
fn pre_register_rpc_port(rpc_registry: &SharedRpcRegistry, mod_name: &str, rpc_port: u16) {
    if let Ok(mut reg) = rpc_registry.write() {
        reg.register(mod_name.to_string(), rpc_port, Default::default());
    }
}

fn launch_mod_service_process(
    service: &ModService,
    rpc_port: u16,
    runtime: &RuntimeResolver,
) -> std::io::Result<std::process::Child> {
    runtime
        .node_command_with_tsx()
        .no_window_process_group()
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg(&service.script_path)
        .current_dir(&service.mods_dir)
        .env("HMCS_MOD_NAME", &service.mod_name)
        .env("HMCS_RPC_PORT", rpc_port.to_string())
        .spawn()
}

/// Wraps a spawned child process in a [`NodeProcessHandle`], spawning
/// background log reader threads for stdout and stderr.
///
/// On Windows, attaches the child to a Job Object so the entire process tree
/// is terminated when the handle is dropped. The `log_prefix` is used as the
/// tag for log lines emitted by the reader threads.
pub fn build_process_handle(mut child: std::process::Child, log_prefix: &str) -> NodeProcessHandle {
    spawn_log_reader(child.stdout.take(), log_prefix, false);
    spawn_log_reader(child.stderr.take(), log_prefix, true);

    #[cfg(windows)]
    let job = homunculus_utils::process::create_job_for_child(&child);
    #[cfg(windows)]
    return NodeProcessHandle::new(child, job);
    #[cfg(not(windows))]
    NodeProcessHandle::new(child)
}

/// Binds `127.0.0.1:0` to let the OS assign an ephemeral port, then returns
/// that port number. The listener is dropped immediately so the port is free
/// for the MOD service process to bind.
fn allocate_ephemeral_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    Ok(listener.local_addr()?.port())
}

/// Spawns a background thread that reads lines from `reader` and forwards
/// them to the Bevy log with a `[{log_prefix}]` tag.
///
/// Stderr lines are logged at `warn` level; stdout at `info`. The thread
/// exits when the reader reaches EOF or encounters an error. A `None`
/// reader is a no-op.
pub fn spawn_log_reader<R: std::io::Read + Send + 'static>(
    reader: Option<R>,
    log_prefix: &str,
    is_stderr: bool,
) {
    let Some(reader) = reader else { return };
    let stream = if is_stderr { "stderr" } else { "stdout" };
    let thread_name = format!("mod-{log_prefix}-{stream}");
    let log_prefix = log_prefix.to_owned();
    let result = std::thread::Builder::new()
        .name(thread_name)
        .spawn(move || {
            let mut reader = BufReader::new(reader);
            let mut buf = Vec::new();
            loop {
                buf.clear();
                match reader.read_until(b'\n', &mut buf) {
                    Ok(0) => break,
                    Ok(_) => {
                        if buf.last() == Some(&b'\n') {
                            buf.pop();
                        }
                        if buf.last() == Some(&b'\r') {
                            buf.pop();
                        }
                        let line = String::from_utf8_lossy(&buf);
                        if is_stderr {
                            warn!(target: "mod", "[{log_prefix}] {line}");
                        } else {
                            info!(target: "mod", "[{log_prefix}] {line}");
                        }
                    }
                    Err(_) => break,
                }
            }
        });

    if let Err(e) = result {
        error!("Failed to spawn log reader thread for mod: {e}");
    }
}
