use bevy::prelude::*;
use std::process::{Child, Command};

/// Handle to a running Node.js child process for a mod's `main` script.
///
/// On Unix, the child is spawned in its own process group so that
/// [`Drop`] can kill the entire group (pnpm → tsx → node chain).
#[derive(Component)]
pub(crate) struct NodeProcessHandle(pub Child);

impl Drop for NodeProcessHandle {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            let pid = self.0.id() as libc::pid_t;
            // Kill the entire process group (negative PID).
            // The child was spawned with process_group(0), so its PID == PGID.
            unsafe {
                libc::kill(-pid, libc::SIGKILL);
            }
        }
        #[cfg(not(unix))]
        {
            let _ = self.0.kill();
        }
    }
}

/// Inserted at startup if Node.js is available on the system.
#[derive(Resource)]
pub(crate) struct NodeAvailable;

pub(crate) struct NodeProcessPlugin;

impl Plugin for NodeProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, check_node_available);
    }
}

fn check_node_available(mut commands: Commands) {
    match Command::new("node").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            info!("Node.js available: {}", version.trim());
            commands.insert_resource(NodeAvailable);
        }
        _ => {
            warn!(
                "Node.js not found. Mod scripts will not run. Install Node.js to enable mod scripting."
            );
        }
    }
}
