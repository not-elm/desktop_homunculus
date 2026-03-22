use crate::node_process::{NodeAvailable, NodeProcessHandle};
use bevy::prelude::*;
use homunculus_core::prelude::SharedRpcRegistry;
use homunculus_utils::process::CommandNoWindow;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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

pub(crate) struct ModServicePlugin;

impl Plugin for ModServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            run_mod_services.run_if(resource_exists::<NodeAvailable>),
        );
    }
}

fn run_mod_services(
    mut commands: Commands,
    services: Query<(Entity, &ModService)>,
    rpc_registry: Res<SharedRpcRegistry>,
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

        match launch_mod_service_process(service, rpc_port) {
            Ok(child) => {
                append_pid_file(child.id());
                commands.spawn(build_process_handle(child, &service.mod_name));
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

/// Append a child PID to `~/.homunculus/mod_pids` so stale processes can be
/// detected and cleaned up on next launch.
fn append_pid_file(pid: u32) {
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
) -> std::io::Result<std::process::Child> {
    Command::new("node")
        .no_window_process_group()
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("--import")
        .arg("tsx")
        .arg(&service.script_path)
        .current_dir(&service.mods_dir)
        .env("HMCS_MOD_NAME", &service.mod_name)
        .env("HMCS_RPC_PORT", rpc_port.to_string())
        .spawn()
}

fn build_process_handle(mut child: std::process::Child, mod_name: &str) -> NodeProcessHandle {
    spawn_log_reader(child.stdout.take(), mod_name, false);
    spawn_log_reader(child.stderr.take(), mod_name, true);

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

fn spawn_log_reader<R: std::io::Read + Send + 'static>(
    reader: Option<R>,
    mod_name: &str,
    is_stderr: bool,
) {
    let Some(reader) = reader else { return };
    let stream = if is_stderr { "stderr" } else { "stdout" };
    let thread_name = format!("mod-{mod_name}-{stream}");
    let mod_name = mod_name.to_owned();
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
                            warn!(target: "mod", "[{mod_name}] {line}");
                        } else {
                            info!(target: "mod", "[{mod_name}] {line}");
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
