//! System tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

/// Windows flag to prevent spawning a visible console window for child processes.
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Maximum allowed timeout for command execution (5 minutes).
const MAX_TIMEOUT_MS: u64 = 300_000;

/// Maximum bytes to read from stdout or stderr (1 MB).
const MAX_OUTPUT_BYTES: u64 = 1_048_576;

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

#[rmcp::tool_router(router = system_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Execute a MOD command.
    #[tool(
        name = "execute_command",
        description = "Execute a short-lived MOD bin command (spawns a new process). Returns stdout, stderr, and exit code. For stateful service RPC, use call_rpc instead. Use 'mods' resource to discover available commands."
    )]
    async fn execute_command(&self, params: Parameters<ExecuteCommandParams>) -> String {
        let args = params.0;
        let timeout_ms = args.timeout_ms.unwrap_or(30_000).min(MAX_TIMEOUT_MS);
        let timeout = Duration::from_millis(timeout_ms);
        let mut cmd = self.create_command(&args);

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

        // Read stdout/stderr concurrently, capped at MAX_OUTPUT_BYTES each.
        let stdout_handle = child.stdout.take();
        let stderr_handle = child.stderr.take();

        let result = tokio::time::timeout(timeout, async {
            let (stdout_buf, stderr_buf) =
                tokio::join!(read_pipe(stdout_handle), read_pipe(stderr_handle));
            let status = child.wait().await;
            (stdout_buf, stderr_buf, status)
        })
        .await;

        let (stdout_buf, stderr_buf, status) = match result {
            Ok(r) => r,
            Err(_) => {
                let _ = child.kill().await;
                return format!(
                    "Error: Command '{}' timed out after {timeout_ms}ms",
                    args.command
                );
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

    fn create_command(&self, args: &ExecuteCommandParams) -> Command {
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
        cmd
    }
}

/// Reads up to [`MAX_OUTPUT_BYTES`] from an optional async reader.
async fn read_pipe(handle: Option<impl AsyncRead + Unpin>) -> Vec<u8> {
    match handle {
        Some(r) => {
            let mut buf = Vec::new();
            let _ = r.take(MAX_OUTPUT_BYTES).read_to_end(&mut buf).await;
            buf
        }
        None => Vec::new(),
    }
}
