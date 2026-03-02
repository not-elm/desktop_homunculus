use bevy::prelude::*;
use std::process::{Child, Command};

/// Handle to a running Node.js child process for a mod's `main` script.
#[derive(Component)]
pub(crate) struct NodeProcessHandle(pub Child);

impl Drop for NodeProcessHandle {
    fn drop(&mut self) {
        let _ = self.0.kill();
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
