//! `/mods` provides endpoints for mod management and command execution.

use axum::Json;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::{ModInfo, ModMenuMetadata};
use homunculus_utils::config::HomunculusConfig;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use utoipa::ToSchema;

/// List all loaded mods.
#[utoipa::path(
    get,
    path = "/",
    tag = "mods",
    responses(
        (status = 200, description = "List of loaded mods", body = Vec<ModInfo>),
    ),
)]
pub async fn list_mods(State(api): State<ModsApi>) -> HttpResult<Vec<ModInfo>> {
    api.list().await.into_http_result()
}

/// Get a single mod by name.
#[utoipa::path(
    get,
    path = "/{mod_name}",
    tag = "mods",
    params(
        ("mod_name" = String, Path, description = "Mod package name"),
    ),
    responses(
        (status = 200, description = "Mod information", body = ModInfo),
        (status = 404, description = "Mod not found"),
    ),
)]
pub async fn get_one(
    State(api): State<ModsApi>,
    Path(mod_name): Path<String>,
) -> HttpResult<ModInfo> {
    api.find_by_name(mod_name).await.into_http_result()
}

/// List all registered mod menus.
#[utoipa::path(
    get,
    path = "/menus",
    tag = "mods",
    responses(
        (status = 200, description = "List of mod menus", body = Vec<ModMenuMetadata>),
    ),
)]
pub async fn list_menus(State(api): State<ModsApi>) -> HttpResult<Vec<ModMenuMetadata>> {
    api.menus().await.into_http_result()
}

/// Request body for `POST /commands/execute`.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandRequest {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub timeout_ms: Option<u64>,
}

/// NDJSON event types emitted during command execution.
#[derive(Serialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CommandEvent {
    Stdout {
        data: String,
    },
    Stderr {
        data: String,
    },
    Exit {
        code: Option<i32>,
        #[serde(rename = "timedOut")]
        timed_out: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        signal: Option<String>,
    },
}

fn validate_request(req: &ExecuteCommandRequest) -> Result<(), ApiError> {
    if req.args.len() > 64 {
        return Err(ApiError::InvalidInput(
            "args must not exceed 64 elements".to_string(),
        ));
    }
    for arg in &req.args {
        if arg.len() > 4096 {
            return Err(ApiError::InvalidInput(
                "each arg must not exceed 4096 characters".to_string(),
            ));
        }
    }
    if let Some(ref stdin) = req.stdin
        && stdin.len() > 1024 * 1024
    {
        return Err(ApiError::InvalidInput(
            "stdin must not exceed 1 MiB".to_string(),
        ));
    }
    if let Some(timeout) = req.timeout_ms
        && !(1..=300_000).contains(&timeout)
    {
        return Err(ApiError::InvalidInput(
            "timeoutMs must be between 1 and 300000".to_string(),
        ));
    }
    Ok(())
}

fn serialize_event(event: &CommandEvent) -> Vec<u8> {
    let mut line = serde_json::to_vec(event).expect("CommandEvent serialization should not fail");
    line.push(b'\n');
    line
}

/// Execute a mod command with NDJSON streaming output.
#[utoipa::path(
    post,
    path = "/execute",
    tag = "commands",
    request_body = ExecuteCommandRequest,
    responses(
        (status = 200, description = "NDJSON stream of command events", content_type = "application/x-ndjson"),
        (status = 400, description = "Invalid request"),
    ),
)]
pub async fn execute_command(
    State(config): State<HomunculusConfig>,
    Json(request): Json<ExecuteCommandRequest>,
) -> Response {
    if let Err(e) = validate_request(&request) {
        return e.into_response();
    }

    let mods_dir = config.mods_dir.clone();
    let command = request.command;
    let timeout = Duration::from_millis(request.timeout_ms.unwrap_or(10_000));
    let stdin_data = request.stdin;
    let args = request.args;

    let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u8>>(64);

    tokio::spawn(async move {
        let mut cmd = tokio::process::Command::new(homunculus_utils::mods::pnpm_program());
        cmd.arg("exec")
            .arg(&command)
            .args(&args)
            .current_dir(&mods_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(if stdin_data.is_some() {
                std::process::Stdio::piped()
            } else {
                std::process::Stdio::null()
            });
        #[cfg(windows)]
        {
            cmd.creation_flags(0x08000000);
            if let Some(path) = homunculus_utils::process::path_with_node_prepended() {
                cmd.env("PATH", path);
            }
        }
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(_) => {
                let _ = tx
                    .send(serialize_event(&CommandEvent::Exit {
                        code: None,
                        timed_out: false,
                        signal: None,
                    }))
                    .await;
                return;
            }
        };

        // Write stdin if provided, then drop to close
        if let Some(data) = stdin_data
            && let Some(mut stdin) = child.stdin.take()
        {
            use tokio::io::AsyncWriteExt;
            let _ = stdin.write_all(data.as_bytes()).await;
            drop(stdin);
        }

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let tx_stdout = tx.clone();
        let stdout_task = tokio::spawn(async move {
            if let Some(stdout) = stdout {
                let reader = tokio::io::BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if tx_stdout
                        .send(serialize_event(&CommandEvent::Stdout { data: line }))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            }
        });

        let tx_stderr = tx.clone();
        let stderr_task = tokio::spawn(async move {
            if let Some(stderr) = stderr {
                let reader = tokio::io::BufReader::new(stderr);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if tx_stderr
                        .send(serialize_event(&CommandEvent::Stderr { data: line }))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
            }
        });

        let timed_out;
        let exit_status;

        tokio::select! {
            status = child.wait() => {
                timed_out = false;
                exit_status = status.ok();
            }
            _ = tokio::time::sleep(timeout) => {
                timed_out = true;
                let _ = child.kill().await;
                exit_status = child.wait().await.ok();
            }
        }

        // Wait for output readers to finish
        let _ = stdout_task.await;
        let _ = stderr_task.await;

        let code = exit_status.and_then(|s| s.code());

        #[cfg(unix)]
        let signal = {
            use std::os::unix::process::ExitStatusExt;
            exit_status.and_then(|s| s.signal()).map(|s| format!("{s}"))
        };
        #[cfg(not(unix))]
        let signal: Option<String> = None;

        let _ = tx
            .send(serialize_event(&CommandEvent::Exit {
                code,
                timed_out,
                signal,
            }))
            .await;
    });

    let stream = ReceiverStream::new(rx);
    let body = Body::from_stream(stream.map(Ok::<_, std::convert::Infallible>));

    Response::builder()
        .header("Content-Type", "application/x-ndjson")
        .header("Cache-Control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .unwrap()
        .into_response()
}
