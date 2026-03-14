use std::convert::Infallible;
use std::time::Duration;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use bevy::tasks::futures_lite::{Stream, StreamExt};
use futures::stream::unfold;
use homunculus_api::stt::{ModelDownloadResponse, ModelInfo, SttApi, SttError, SttStartResponse};
use homunculus_microphone::{
    SttModelSize,
    session::{SttEvent, SttStartOptions, SttState},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// SSE keepalive interval in seconds.
const SSE_KEEPALIVE_SECS: u64 = 30;

/// Request body for model download.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelDownloadRequest {
    pub model_size: SttModelSize,
}

/// SSE payload for STT recognition results.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SttResultPayload<'a> {
    text: &'a str,
    timestamp: f64,
    language: &'a str,
}

/// SSE payload for STT session errors.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SttSessionErrorPayload<'a> {
    error: &'a str,
    message: &'a str,
}

/// Start an STT session.
#[utoipa::path(
    post,
    path = "/start",
    tag = "stt",
    request_body(content = Option<SttStartOptions>, content_type = "application/json"),
    responses(
        (status = 200, description = "Session started", body = SttStartResponse),
        (status = 409, description = "Session already active or loading"),
        (status = 412, description = "Model not available"),
        (status = 422, description = "Invalid language"),
        (status = 500, description = "Internal server error"),
        (status = 503, description = "Microphone unavailable"),
    ),
)]
pub async fn start(
    State(api): State<SttApi>,
    body: Option<Json<SttStartOptions>>,
) -> Result<Json<SttStartResponse>, SttErrorResponse> {
    let options = body.map(|b| b.0).unwrap_or_default();
    let response = api.start(options).await?;
    Ok(Json(response))
}

/// Stop the current STT session. Idempotent.
#[utoipa::path(
    post,
    path = "/stop",
    tag = "stt",
    responses(
        (status = 200, description = "Session stopped", body = SttState),
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
        (status = 200, description = "Current session state", body = SttState),
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
    let (current_state, rx) = api.subscribe().await;

    let initial = futures::stream::once(async move {
        Ok(stt_event_to_sse(&SttEvent::Status(current_state)))
    });

    let ongoing = unfold(rx, |mut rx| async move {
        let event = rx.recv().await.ok()?;
        Some((Ok(stt_event_to_sse(&event)), rx))
    });

    let disconnected = futures::stream::once(async {
        Ok(Event::default().event("disconnected").data("{}"))
    });

    Sse::new(initial.chain(ongoing).chain(disconnected))
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(SSE_KEEPALIVE_SECS)))
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

/// Convert an `SttEvent` into an SSE `Event`.
fn stt_event_to_sse(event: &SttEvent) -> Event {
    match event {
        SttEvent::Status(state) => Event::default()
            .event("status")
            .data(serde_json::to_string(state).unwrap_or_else(|e| {
                bevy::log::error!("SSE serialization failed: {e}");
                "{}".to_string()
            })),
        SttEvent::Result {
            text,
            timestamp,
            language,
        } => Event::default().event("result").data(
            serde_json::to_string(&SttResultPayload {
                text,
                timestamp: *timestamp,
                language,
            })
            .unwrap_or_else(|e| {
                bevy::log::error!("SSE serialization failed: {e}");
                "{}".to_string()
            }),
        ),
        SttEvent::SessionError { error, message } => Event::default().event("session_error").data(
            serde_json::to_string(&SttSessionErrorPayload { error, message }).unwrap_or_else(
                |e| {
                    bevy::log::error!("SSE serialization failed: {e}");
                    "{}".to_string()
                },
            ),
        ),
        SttEvent::Stopped => Event::default().event("stopped").data("{}"),
    }
}

/// Wrapper for converting `SttError` into HTTP responses.
///
/// STT needs domain-specific HTTP status codes (409 conflict, 412 precondition,
/// 503 unavailable) that don't map to the shared `ApiError` variants.
/// Long-term, `ApiError` could be extended with a `Domain` variant.
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
            SttError::SessionLoading => (StatusCode::CONFLICT, "session_loading"),
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
            SttError::InvalidLanguage(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_language"),
            SttError::InvalidModelSize => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_model_size"),
        };

        let body = serde_json::json!({
            "error": error_code,
            "message": self.0.to_string(),
        });

        (status, Json(body)).into_response()
    }
}
