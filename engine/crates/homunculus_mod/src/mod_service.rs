use crate::node_process::{NodeAvailable, NodeProcessHandle};
use bevy::prelude::*;
use homunculus_utils::process::CommandNoWindow;
use std::path::PathBuf;
use std::process::Command;

/// A MOD service identified by its absolute filesystem path.
///
/// Services are long-running Node.js child processes that run for the
/// entire app session, declared via the `homunculus.service` field in a MOD's `package.json`.
#[derive(Component)]
pub(crate) struct ModService {
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

fn run_mod_services(mut commands: Commands, services: Query<(Entity, &ModService)>) {
    for (entity, service) in services.iter() {
        info!("Starting mod service: {}", service.script_path.display());
        let mut cmd = Command::new("node");
        cmd.no_window_process_group()
            .arg("--import")
            .arg("tsx")
            .arg(&service.script_path)
            .current_dir(&service.mods_dir);

        match cmd.spawn() {
            Ok(child) => {
                append_pid_file(child.id());

                // Create a Job Object for the child process tree (Windows only).
                #[cfg(windows)]
                let job = homunculus_utils::process::create_job_for_child(&child);

                #[cfg(windows)]
                let handle = NodeProcessHandle::new(child, job);
                #[cfg(not(windows))]
                let handle = NodeProcessHandle::new(child);

                commands.spawn(handle);
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
