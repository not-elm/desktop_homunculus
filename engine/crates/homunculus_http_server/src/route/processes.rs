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
pub async fn list_processes(State(api): State<ProcessesApi>) -> HttpResult<Vec<ProcessInfo>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_response, call_any_status, test_app};
    use axum::body::Body;
    use axum::http::Request;

    #[tokio::test]
    async fn test_list_processes_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/processes").body(Body::empty()).unwrap();
        assert_response::<Vec<ProcessInfo>>(&mut app, router, request, vec![]).await;
    }

    #[tokio::test]
    async fn test_start_invalid_command_format() {
        let (mut app, router) = test_app();
        let request = Request::post("/processes/start")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"command":"no-colon-here"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        // Invalid command format is surfaced from spawn_managed_process via InvalidInput → 400
        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_start_nonexistent_mod() {
        let (mut app, router) = test_app();
        let request = Request::post("/processes/start")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"command":"nonexistent:cmd"}"#))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        // Mod not found → InvalidInput → 400
        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_stop_nonexistent_handle() {
        let (mut app, router) = test_app();
        let request = Request::delete("/processes/nonexistent-id")
            .body(Body::empty())
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        // EntityNotFound → 404
        assert_eq!(response.status(), 404);
    }

    #[tokio::test]
    async fn test_start_args_too_many() {
        let (mut app, router) = test_app();
        let args: Vec<String> = (0..65).map(|i| format!("arg{i}")).collect();
        let body = serde_json::json!({ "command": "test:cmd", "args": args });
        let request = Request::post("/processes/start")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        // Validation error → 400
        assert_eq!(response.status(), 400);
    }

    #[tokio::test]
    async fn test_start_arg_too_long() {
        let (mut app, router) = test_app();
        let long_arg = "x".repeat(4097);
        let body = serde_json::json!({ "command": "test:cmd", "args": [long_arg] });
        let request = Request::post("/processes/start")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap();
        let response = call_any_status(&mut app, router, request).await;
        // Validation error → 400
        assert_eq!(response.status(), 400);
    }
}
