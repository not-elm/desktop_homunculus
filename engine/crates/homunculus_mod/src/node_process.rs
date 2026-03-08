use bevy::prelude::*;
use homunculus_utils::process::CommandNoWindow;
use std::process::{Child, Command};
use std::time::Duration;

/// Handle to a running Node.js child process for a mod's `main` script.
///
/// The child is spawned as a single `node --import tsx` process.
/// [`Drop`] performs best-effort cleanup (kill + reap).
/// For graceful shutdown, use [`NodeProcessHandle::shutdown`].
#[derive(Component)]
pub(crate) struct NodeProcessHandle(pub Child);

impl NodeProcessHandle {
    /// Gracefully shut down the child process.
    ///
    /// Sends SIGTERM, waits up to `grace` for the process to exit,
    /// then falls back to SIGKILL + wait.
    pub(crate) fn shutdown(&mut self, _grace: Duration) {
        if let Ok(Some(_)) = self.0.try_wait() {
            return;
        }

        #[cfg(not(unix))]
        {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }

        #[cfg(unix)]
        {
            unsafe {
                libc::kill(self.0.id() as libc::pid_t, libc::SIGTERM);
            }
            let deadline = std::time::Instant::now() + _grace;
            while std::time::Instant::now() < deadline {
                if let Ok(Some(_)) = self.0.try_wait() {
                    return;
                }
                std::thread::sleep(Duration::from_millis(25));
            }

            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

impl Drop for NodeProcessHandle {
    fn drop(&mut self) {
        // Best-effort cleanup: kill + reap to prevent zombie.
        let _ = self.0.kill();
        let _ = self.0.try_wait();
    }
}

/// Inserted at startup if Node.js is available on the system.
#[derive(Resource)]
pub(crate) struct NodeAvailable;

pub(crate) struct NodeProcessPlugin;

impl Plugin for NodeProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (cleanup_stale_mod_processes, check_node_available).chain(),
        );
        app.add_systems(Last, cleanup_node_processes.run_if(on_message::<AppExit>));
    }
}

/// Kill leftover mod service processes from a previous session that did not
/// shut down cleanly (e.g. crash or SIGKILL).
///
/// Reads PIDs from `~/.homunculus/mod_pids`, verifies each is still a
/// `node --import` process, and sends SIGTERM (then SIGKILL after 500 ms).
fn cleanup_stale_mod_processes() {
    let pid_path = homunculus_utils::path::homunculus_dir().join("mod_pids");
    let Ok(content) = std::fs::read_to_string(&pid_path) else {
        return;
    };

    // Remove immediately — a fresh file is created for the new session.
    let _ = std::fs::remove_file(&pid_path);

    for line in content.lines() {
        let Ok(pid) = line.trim().parse::<u32>() else {
            continue;
        };
        kill_if_mod_service(pid);
    }
}

#[cfg(unix)]
fn kill_if_mod_service(pid: u32) {
    // Check if the process is still alive.
    let alive = unsafe { libc::kill(pid as libc::pid_t, 0) } == 0;
    if !alive {
        return;
    }

    // Verify the command line looks like a mod service (`node --import`).
    let Ok(output) = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "command="])
        .output()
    else {
        return;
    };

    let cmdline = String::from_utf8_lossy(&output.stdout);
    if !cmdline.contains("node") || !cmdline.contains("--import") {
        return;
    }

    info!("Killing stale mod service process: pid={pid}");
    unsafe {
        libc::kill(pid as libc::pid_t, libc::SIGTERM);
    }

    // Wait briefly for graceful exit before escalating to SIGKILL.
    let deadline = Instant::now() + Duration::from_millis(500);
    while Instant::now() < deadline {
        if unsafe { libc::kill(pid as libc::pid_t, 0) } != 0 {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    unsafe {
        libc::kill(pid as libc::pid_t, libc::SIGKILL);
    }
}

#[cfg(not(unix))]
fn kill_if_mod_service(_pid: u32) {
    // Windows: best-effort, skip for now.
}

fn check_node_available(mut commands: Commands) {
    match Command::new("node").no_window().arg("--version").output() {
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

/// Gracefully shuts down all child processes when AppExit is triggered.
fn cleanup_node_processes(mut handles: Query<&mut NodeProcessHandle>) {
    for mut handle in handles.iter_mut() {
        handle.shutdown(Duration::from_secs(2));
    }
    // Clean exit — remove PID file so next launch won't try to kill these.
    let pid_path = homunculus_utils::path::homunculus_dir().join("mod_pids");
    let _ = std::fs::remove_file(&pid_path);
}
