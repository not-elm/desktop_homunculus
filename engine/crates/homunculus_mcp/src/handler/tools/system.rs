//! System tool implementations for the MCP handler.

use std::process::Stdio;
use std::time::Duration;

use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use super::super::HomunculusMcpHandler;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Windows flag to prevent spawning a visible console window for child processes.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

/// Parameters for the `execute_command` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandParams {
    /// The MOD command to execute (e.g. "voicevox").
    pub command: String,
    /// Additional arguments to pass to the command.
    pub args: Option<Vec<String>>,
    /// Optional data to write to the command's stdin.
    pub stdin: Option<String>,
    /// Timeout in milliseconds (default: 30000).
    pub timeout_ms: Option<u64>,
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = system_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Execute a MOD command.
    #[tool(
        name = "execute_command",
        description = "Execute a MOD command (e.g., VoiceVox speak, initialize). Returns stdout, stderr, and exit code. Use 'mods' resource to discover available commands."
    )]
    async fn execute_command(&self, params: Parameters<ExecuteCommandParams>) -> String {
        let args = params.0;
        let timeout_ms = args.timeout_ms.unwrap_or(30000);
        let timeout = Duration::from_millis(timeout_ms);

        let mut cmd = Command::new(homunculus_utils::mods::pnpm_program());
        cmd.arg("exec")
            .arg(&args.command)
            .current_dir(&self.config.mods_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(ref extra_args) = args.args {
            cmd.args(extra_args);
        }

        if args.stdin.is_some() {
            cmd.stdin(Stdio::piped());
        } else {
            cmd.stdin(Stdio::null());
        }

        #[cfg(windows)]
        cmd.creation_flags(CREATE_NO_WINDOW);

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => return format!("Error: Failed to spawn command: {e}"),
        };

        // Write stdin if provided, then drop to close.
        if let Some(data) = &args.stdin
            && let Some(mut stdin) = child.stdin.take()
        {
            let _ = stdin.write_all(data.as_bytes()).await;
            drop(stdin);
        }

        // Read stdout/stderr concurrently while waiting for exit.
        let stdout_handle = child.stdout.take();
        let stderr_handle = child.stderr.take();

        let read_all = async {
            let stdout_task = async {
                if let Some(mut r) = stdout_handle {
                    let mut buf = Vec::new();
                    tokio::io::AsyncReadExt::read_to_end(&mut r, &mut buf)
                        .await
                        .ok();
                    buf
                } else {
                    Vec::new()
                }
            };
            let stderr_task = async {
                if let Some(mut r) = stderr_handle {
                    let mut buf = Vec::new();
                    tokio::io::AsyncReadExt::read_to_end(&mut r, &mut buf)
                        .await
                        .ok();
                    buf
                } else {
                    Vec::new()
                }
            };
            let (stdout_buf, stderr_buf) = tokio::join!(stdout_task, stderr_task);
            let status = child.wait().await;
            (stdout_buf, stderr_buf, status)
        };

        let (stdout_buf, stderr_buf, status) = tokio::select! {
            result = read_all => result,
            _ = tokio::time::sleep(timeout) => {
                // `child` is moved into `read_all`, but select drops the losing future.
                return format!("Error: Command '{}' timed out after {}ms", args.command, timeout_ms);
            }
        };

        let stdout = String::from_utf8_lossy(&stdout_buf);
        let stderr = String::from_utf8_lossy(&stderr_buf);
        let code = match status {
            Ok(s) => s.code(),
            Err(e) => return format!("Error: Failed to wait for command: {e}"),
        };

        let mut result = format!(
            "stdout:\n{stdout}\n\nstderr:\n{stderr}\n\nexit code: {}",
            code.map_or("unknown".to_string(), |c| c.to_string())
        );

        if code.is_some_and(|c| c != 0) {
            result = format!("Error: Command exited with non-zero status.\n\n{result}");
        }

        result
    }
}
