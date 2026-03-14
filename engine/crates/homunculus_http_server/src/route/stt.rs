use std::convert::Infallible;
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use bevy::tasks::futures_lite::{Stream, StreamExt};
use futures::stream::unfold;
use homunculus_api::stt::{ModelDownloadResponse, ModelInfo, SttApi, SttError};
use homunculus_microphone::{
    SttModelSize,
    session::{SttEvent, SttStartOptions, SttState},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request body for model download.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelDownloadRequest {
    pub model_size: SttModelSize,
}

/// Start an STT session.
#[utoipa::path(
    post,
    path = "/start",
    tag = "stt",
    request_body(content = SttStartOptions, content_type = "application/json"),
    responses(
        (status = 200, description = "Session started"),
        (status = 409, description = "Session already active"),
        (status = 412, description = "Model not available"),
        (status = 500, description = "Internal server error"),
        (status = 503, description = "Microphone unavailable"),
    ),
)]
pub async fn start(
    State(api): State<SttApi>,
    body: Option<Json<SttStartOptions>>,
) -> Result<Json<SttState>, SttErrorResponse> {
    let options = body.map(|b| b.0).unwrap_or_default();
    let state = api.start(options).await?;
    Ok(Json(state))
}

/// Stop the current STT session. Idempotent.
#[utoipa::path(
    post,
    path = "/stop",
    tag = "stt",
    responses(
        (status = 200, description = "Session stopped"),
    ),
)]
pub async fn stop(State(api): State<SttApi>) -> Json<SttState> {
    Json(api.stop().await)
}

/// Get the current STT session status.
#[utoipa::path(
    get,
    path = "/status",
    tag = "stt",
    responses(
        (status = 200, description = "Current session state"),
    ),
)]
pub async fn status(State(api): State<SttApi>) -> Json<SttState> {
    Json(api.status().await)
}

/// Stream STT events via SSE.
///
/// Events: `status`, `result`, `session_error`, `stopped`.
/// Sends current status on connect (late-join sync).
#[utoipa::path(
    get,
    path = "/stream",
    tag = "stt",
    responses(
        (status = 200, description = "SSE event stream", content_type = "text/event-stream"),
    ),
)]
pub async fn stream(
    State(api): State<SttApi>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>> + Send> {
    let current_state = api.current_state().await;
    let rx = api.new_event_receiver().await;

    let initial = futures::stream::once(async move {
        Ok(Event::default()
            .event("status")
            .data(serde_json::to_string(&current_state).unwrap()))
    });

    let ongoing = unfold(rx, |mut rx| async move {
        let event = rx.recv().await.ok()?;
        let sse_event = match &event {
            SttEvent::Status(state) => Event::default()
                .event("status")
                .data(serde_json::to_string(state).unwrap()),
            SttEvent::Result {
                text,
                timestamp,
                language,
            } => {
                let data = serde_json::json!({
                    "text": text,
                    "timestamp": timestamp,
                    "language": language,
                });
                Event::default()
                    .event("result")
                    .data(serde_json::to_string(&data).unwrap())
            }
            SttEvent::SessionError { error, message } => {
                let data = serde_json::json!({
                    "error": error,
                    "message": message,
                });
                Event::default()
                    .event("session_error")
                    .data(serde_json::to_string(&data).unwrap())
            }
            SttEvent::Stopped => Event::default().event("stopped").data("{}"),
        };
        Some((Ok(sse_event), rx))
    });

    Sse::new(initial.chain(ongoing)).keep_alive(KeepAlive::new().interval(Duration::from_secs(30)))
}

/// Download an STT model.
#[utoipa::path(
    post,
    path = "/models/download",
    tag = "stt",
    request_body = ModelDownloadRequest,
    responses(
        (status = 200, description = "Download result"),
        (status = 422, description = "Invalid model size"),
        (status = 500, description = "Download failed"),
    ),
)]
pub async fn download_model(
    State(api): State<SttApi>,
    Json(body): Json<ModelDownloadRequest>,
) -> Result<Json<ModelDownloadResponse>, SttErrorResponse> {
    let response = api.download_model(body.model_size).await?;
    Ok(Json(response))
}

/// List downloaded STT models.
#[utoipa::path(
    get,
    path = "/models",
    tag = "stt",
    responses(
        (status = 200, description = "List of available models", body = Vec<ModelInfo>),
    ),
)]
pub async fn list_models(State(api): State<SttApi>) -> Json<Vec<ModelInfo>> {
    Json(api.list_models())
}

/// Wrapper for converting SttError into HTTP responses.
pub struct SttErrorResponse(SttError);

impl From<SttError> for SttErrorResponse {
    fn from(err: SttError) -> Self {
        Self(err)
    }
}

impl IntoResponse for SttErrorResponse {
    fn into_response(self) -> Response {
        let (status, error_code) = match &self.0 {
            SttError::SessionAlreadyActive => (StatusCode::CONFLICT, "session_already_active"),
            SttError::ModelNotAvailable(_) => {
                (StatusCode::PRECONDITION_FAILED, "model_not_available")
            }
            SttError::ModelLoadFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "model_load_failed")
            }
            SttError::PipelineFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "pipeline_failed"),
            SttError::NoMicrophone => (StatusCode::SERVICE_UNAVAILABLE, "no_microphone"),
            SttError::MicrophonePermissionDenied => (
                StatusCode::SERVICE_UNAVAILABLE,
                "microphone_permission_denied",
            ),
            SttError::DownloadFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "download_failed"),
            SttError::InvalidModelSize => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_model_size"),
        };

        let body = serde_json::json!({
            "error": error_code,
            "message": self.0.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
