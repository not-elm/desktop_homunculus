use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(windows)]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

/// Extension trait to suppress console window creation on Windows.
///
/// On non-Windows platforms, all methods are no-ops.
pub trait CommandNoWindow {
    /// Suppress the console window on Windows.
    fn no_window(&mut self) -> &mut Self;

    /// Suppress the console window and create a new process group on Windows.
    ///
    /// The process group enables `GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT)`
    /// for graceful shutdown. Use this for long-running child processes
    /// (mod services) that need lifecycle management.
    ///
    /// **Important**: `CommandExt::creation_flags` *replaces* (not ORs) flags
    /// on each call. Both flags are combined here in a single call. Callers
    /// must not call `creation_flags` independently after this method.
    fn no_window_process_group(&mut self) -> &mut Self;
}

impl CommandNoWindow for Command {
    fn no_window(&mut self) -> &mut Self {
        #[cfg(windows)]
        self.creation_flags(CREATE_NO_WINDOW);
        self
    }

    fn no_window_process_group(&mut self) -> &mut Self {
        #[cfg(windows)]
        self.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);
        self
    }
}

// ── Windows Job Object support ──────────────────────────────────────────────

/// RAII wrapper for a Windows Job Object handle.
///
/// Closes the handle on drop, which triggers `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`
/// if that limit was set — terminating all processes in the job.
#[cfg(windows)]
pub struct JobHandle(windows_sys::Win32::Foundation::HANDLE);

// SAFETY: The Job Object HANDLE is a kernel object that can be used from any thread.
#[cfg(windows)]
unsafe impl Send for JobHandle {}
// SAFETY: JobHandle is only accessed via &mut self (Drop) or consumed.
#[cfg(windows)]
unsafe impl Sync for JobHandle {}

#[cfg(windows)]
impl Drop for JobHandle {
    fn drop(&mut self) {
        // SAFETY: self.0 is a valid handle obtained from CreateJobObjectW.
        // Double-close is harmless (returns ERROR_INVALID_HANDLE, which we ignore).
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

/// Create an anonymous Job Object with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` and
/// assign the given child process to it.
///
/// Returns `Some(JobHandle)` on success, `None` if any step fails (with a warning
/// logged). The caller should store the handle to keep the job alive; dropping it
/// terminates all processes in the job.
#[cfg(windows)]
pub fn create_job_for_child(child: &std::process::Child) -> Option<JobHandle> {
    use std::mem::{size_of, zeroed};
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
        SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    };
    use windows_sys::Win32::System::Threading::OpenProcess;

    // Create an anonymous Job Object.
    // SAFETY: Both arguments are null for an anonymous, unsecured job.
    let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
    if job.is_null() {
        log::warn!("Failed to create Job Object for mod service process");
        return None;
    }

    // Set KILL_ON_JOB_CLOSE so the OS kills all processes when the handle closes.
    // SAFETY: job is a valid handle from CreateJobObjectW. The struct is zeroed
    // and only LimitFlags is set, which is the documented usage pattern.
    let result = unsafe {
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const _,
            size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        )
    };
    if result == 0 {
        log::warn!("Failed to set JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE");
        // SAFETY: job is a valid handle.
        unsafe { CloseHandle(job) };
        return None;
    }

    // Open a handle to the child process for job assignment.
    // SAFETY: child.id() returns a valid PID.
    let process_handle = unsafe {
        OpenProcess(
            windows_sys::Win32::System::Threading::PROCESS_SET_QUOTA
                | windows_sys::Win32::System::Threading::PROCESS_TERMINATE,
            0, // bInheritHandle = FALSE
            child.id(),
        )
    };
    if process_handle.is_null() {
        // Process may have already exited.
        log::warn!(
            "Failed to open process handle for pid={} (process may have exited)",
            child.id()
        );
        // SAFETY: job is a valid handle.
        unsafe { CloseHandle(job) };
        return None;
    }

    // Assign the child process to the job.
    // SAFETY: Both handles are valid. On failure (e.g., nested job conflict),
    // we degrade gracefully.
    let result = unsafe { AssignProcessToJobObject(job, process_handle) };
    // SAFETY: process_handle is a valid handle from OpenProcess.
    unsafe { CloseHandle(process_handle) };
    if result == 0 {
        log::warn!(
            "Failed to assign process pid={} to Job Object (nested job conflict?). \
             Falling back to PID-file tracking.",
            child.id()
        );
        // SAFETY: job is a valid handle.
        unsafe { CloseHandle(job) };
        return None;
    }

    Some(JobHandle(job))
}

/// Terminate all processes in a Job Object with exit code 1.
///
/// Used during forceful shutdown after the grace period expires.
#[cfg(windows)]
pub fn terminate_job(job: &JobHandle) {
    // SAFETY: job.0 is a valid Job Object handle.
    unsafe {
        windows_sys::Win32::System::JobObjects::TerminateJobObject(job.0, 1);
    }
}

/// Send `CTRL_BREAK_EVENT` to a process group.
///
/// The process must have been spawned with `CREATE_NEW_PROCESS_GROUP`.
/// Returns `true` if the event was sent successfully.
#[cfg(windows)]
pub fn send_ctrl_break(pid: u32) -> bool {
    use windows_sys::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT};
    // SAFETY: pid is a valid process group ID (same as PID when spawned
    // with CREATE_NEW_PROCESS_GROUP). CTRL_BREAK_EVENT works with nonzero
    // dwProcessGroupId (unlike CTRL_C_EVENT which is silently ignored).
    unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid) != 0 }
}

/// Check if a stale PID belongs to a `node.exe` process and kill it.
///
/// Verifies: (1) PID is alive, (2) executable is `node.exe`.
/// This is the Windows equivalent of the Unix `kill_if_mod_service`.
#[cfg(windows)]
pub fn kill_if_mod_service_windows(pid: u32) {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, TerminateProcess, WaitForSingleObject,
        PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
    };

    // Try to open the process — if this fails, the PID is not alive.
    // SAFETY: pid is a u32 PID read from the PID file.
    let handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_TERMINATE,
            0,
            pid,
        )
    };
    if handle.is_null() {
        return; // Process not found — already exited.
    }

    // Verify the executable is node.exe.
    let is_node = unsafe {
        let mut buf = [0u16; 260]; // MAX_PATH
        let mut len = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut len);
        if ok != 0 {
            let path = String::from_utf16_lossy(&buf[..len as usize]);
            let filename = path.rsplit(['\\', '/']).next().unwrap_or("");
            filename.eq_ignore_ascii_case("node.exe")
        } else {
            false
        }
    };

    if !is_node {
        // SAFETY: handle is a valid process handle from OpenProcess.
        unsafe { CloseHandle(handle) };
        return; // Not a node process — PID was reused by something else.
    }

    log::info!("Killing stale mod service process: pid={pid}");

    // SAFETY: handle is a valid process handle with PROCESS_TERMINATE access.
    unsafe {
        TerminateProcess(handle, 1);
        // Wait briefly for the process to actually terminate (up to 500ms).
        WaitForSingleObject(handle, 500);
        CloseHandle(handle);
    }
}
