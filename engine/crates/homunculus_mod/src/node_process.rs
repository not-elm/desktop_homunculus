use bevy::prelude::*;
use homunculus_utils::process::CommandNoWindow;
use std::process::{Child, Command};
use std::time::Duration;

/// Handle to a running Node.js child process for a mod's `main` script.
///
/// The child is spawned as a single `node --import tsx` process.
/// On Windows, an optional Job Object handle ensures the entire process tree
/// (including tsx grandchildren) is terminated when the handle is closed.
///
/// [`Drop`] performs best-effort cleanup (kill + reap).
/// For graceful shutdown, use [`NodeProcessHandle::shutdown`].
#[derive(Component)]
pub struct NodeProcessHandle {
    child: Child,
    #[cfg(windows)]
    job: Option<homunculus_utils::process::JobHandle>,
}

impl NodeProcessHandle {
    /// Create a new handle wrapping a child process.
    #[cfg(not(windows))]
    pub fn new(child: Child) -> Self {
        Self { child }
    }

    /// Create a new handle wrapping a child process with an optional Job Object.
    #[cfg(windows)]
    pub fn new(child: Child, job: Option<homunculus_utils::process::JobHandle>) -> Self {
        Self { child, job }
    }

    /// Check if the child process has exited without blocking.
    pub fn try_wait_exited(&mut self) -> Option<std::process::ExitStatus> {
        self.child.try_wait().ok().flatten()
    }

    /// Get the process ID.
    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    /// Shut down the child process.
    ///
    /// On Unix: sends SIGTERM, waits up to `grace` for the process to exit,
    /// then falls back to SIGKILL + wait.
    ///
    /// On Windows: sends CTRL_BREAK_EVENT as a best-effort signal and returns
    /// immediately — does **not** block. Actual termination is delegated to
    /// [`Drop`], which closes the Job Object handle (triggering
    /// `KILL_ON_JOB_CLOSE`). When no Job Object is available, falls back to
    /// `kill()` + `try_wait()` inline.
    pub fn shutdown(&mut self, _grace: Duration) {
        if let Ok(Some(_)) = self.child.try_wait() {
            return;
        }

        #[cfg(unix)]
        {
            unsafe {
                libc::kill(self.child.id() as libc::pid_t, libc::SIGTERM);
            }
            let deadline = std::time::Instant::now() + _grace;
            while std::time::Instant::now() < deadline {
                if let Ok(Some(_)) = self.child.try_wait() {
                    return;
                }
                std::thread::sleep(Duration::from_millis(25));
            }

            let _ = self.child.kill();
            let _ = self.child.wait();
        }

        #[cfg(windows)]
        {
            // Best-effort graceful signal — may not be delivered in release
            // builds where the parent is a GUI-subsystem process and the child
            // was spawned with CREATE_NO_WINDOW (no shared console).
            let _ = homunculus_utils::process::send_ctrl_break(self.child.id());

            // Do NOT busy-wait for the process to exit. Actual termination is
            // handled by Drop (Job Object KILL_ON_JOB_CLOSE). When no Job
            // Object is available, kill the direct child immediately.
            if self.job.is_none() {
                let _ = self.child.kill();
                // Non-blocking: Drop will reap if the process is still running.
                let _ = self.child.try_wait();
            }
        }

        #[cfg(not(any(unix, windows)))]
        {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

impl Drop for NodeProcessHandle {
    fn drop(&mut self) {
        // Best-effort cleanup: close job (kills tree via KILL_ON_JOB_CLOSE),
        // then kill direct child, then reap.
        #[cfg(windows)]
        {
            // Drop the job handle first — CloseHandle triggers
            // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, terminating the entire tree.
            // If shutdown() already closed it, this is harmless.
            self.job.take();
        }

        // Kill direct child (redundant if job killed it; necessary if no job).
        let _ = self.child.kill();
        let _ = self.child.try_wait();
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
///
/// On Windows with Job Objects, stale cleanup is typically unnecessary
/// (the OS kills the tree when the job handle closes at crash time), but
/// this remains as a fallback for processes where `AssignProcessToJobObject` failed.
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
    use std::time::{Duration, Instant};
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

#[cfg(windows)]
fn kill_if_mod_service(pid: u32) {
    homunculus_utils::process::kill_if_mod_service_windows(pid);
}

#[cfg(not(any(unix, windows)))]
fn kill_if_mod_service(_pid: u32) {
    // Unsupported platform — best-effort, skip.
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
