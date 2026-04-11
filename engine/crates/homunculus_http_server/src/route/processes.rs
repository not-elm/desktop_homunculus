use axum::Json;
use axum::extract::{Path, State};
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::processes::{
    ProcessInfo, ProcessesApi, StartProcessRequest, StartProcessResponse,
};

/// Start a managed process.
#[utoipa::path(
    post,
    path = "/start",
    tag = "processes",
    request_body = StartProcessRequest,
    responses(
        (status = 200, description = "Process started", body = StartProcessResponse),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Command not found"),
        (status = 429, description = "Too many processes"),
    ),
)]
pub async fn start(
    State(api): State<ProcessesApi>,
    Json(body): Json<StartProcessRequest>,
) -> HttpResult<StartProcessResponse> {
    validate_start_request(&body)?;
    api.start(body).await.into_http_result()
}

/// Stop a managed process.
#[utoipa::path(
    delete,
    path = "/{handle_id}",
    tag = "processes",
    params(
        ("handle_id" = String, Path, description = "Process handle ID"),
    ),
    responses(
        (status = 200, description = "Process stopped"),
        (status = 404, description = "Handle not found"),
    ),
)]
pub async fn stop(
    State(api): State<ProcessesApi>,
    Path(handle_id): Path<String>,
) -> HttpResult<()> {
    api.stop(handle_id).await.into_http_result()
}

/// List all running managed processes.
#[utoipa::path(
    get,
    path = "/",
    tag = "processes",
    responses(
        (status = 200, description = "List of running processes", body = Vec<ProcessInfo>),
    ),
)]
pub async fn list(State(api): State<ProcessesApi>) -> HttpResult<Vec<ProcessInfo>> {
    api.list().await.into_http_result()
}

fn validate_start_request(req: &StartProcessRequest) -> Result<(), ApiError> {
    if req.args.len() > 64 {
        return Err(ApiError::InvalidInput(
            "args must not exceed 64 elements".into(),
        ));
    }
    for arg in &req.args {
        if arg.len() > 4096 {
            return Err(ApiError::InvalidInput(
                "each arg must not exceed 4096 characters".into(),
            ));
        }
    }
    Ok(())
}
