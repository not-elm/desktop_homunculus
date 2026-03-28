use std::convert::Infallible;
use std::time::Duration;

use axum::Json;
use axum::body::Body;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use bevy::tasks::futures_lite::StreamExt;
use homunculus_api::stt::{ModelDownloadResponse, ModelInfo, RecognizeOptions, SttApi, SttError};
use homunculus_microphone::SttModelSize;
use homunculus_microphone::SttResult;
use homunculus_microphone::model::model_path;
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::ReceiverStream;
use utoipa::ToSchema;

/// Request body for model download.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModelDownloadRequest {
    pub model_size: SttModelSize,
}

/// Recognize a single sentence from the microphone.
///
/// Starts a cpal capture -> VAD -> Whisper pipeline, returns the first
/// recognized sentence, then destroys the pipeline. Long-polls until
/// speech is detected or timeout (60s).
#[utoipa::path(
    post,
    path = "/recognize",
    tag = "stt",
    request_body = RecognizeOptions,
    responses(
        (status = 200, description = "Recognized text", body = SttResult),
        (status = 408, description = "Timeout — no speech detected"),
        (status = 422, description = "Invalid language or model size"),
        (status = 503, description = "Model not available or microphone error"),
    )
)]
pub async fn recognize(
    State(api): State<SttApi>,
    Json(options): Json<RecognizeOptions>,
) -> Result<Json<SttResult>, SttErrorResponse> {
    let result = api.recognize(options).await?;
    Ok(Json(result))
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

/// Query params for cancel download endpoint.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelDownloadQuery {
    pub model_size: Option<SttModelSize>,
}

/// Response for cancel download endpoint.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CancelDownloadResponse {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_size: Option<SttModelSize>,
    cancelled_count: usize,
}

/// Cancel an in-progress model download.
///
/// With `modelSize` query param: cancel a specific download.
/// Without: cancel all in-progress downloads.
#[utoipa::path(
    delete,
    path = "/models/download",
    tag = "stt",
    params(
        ("modelSize" = Option<SttModelSize>, Query, description = "Model size to cancel. Omit to cancel all."),
    ),
    responses(
        (status = 200, description = "Download(s) cancelled"),
        (status = 404, description = "No active download for the specified model"),
    ),
)]
pub async fn cancel_download(
    State(api): State<SttApi>,
    Query(query): Query<CancelDownloadQuery>,
) -> Response {
    if let Some(size) = query.model_size {
        cancel_specific_download(&api, size).await
    } else {
        cancel_all_downloads(&api).await
    }
}

async fn cancel_specific_download(api: &SttApi, size: SttModelSize) -> Response {
    let cancelled = api.cancel_download(size).await;
    if cancelled {
        let body = CancelDownloadResponse {
            status: "cancelled",
            model_size: Some(size),
            cancelled_count: 1,
        };
        (StatusCode::OK, Json(body)).into_response()
    } else {
        let body = serde_json::json!({
            "error": "no_active_download",
            "message": format!("No active download for model size '{}'", size.as_str()),
        });
        (StatusCode::NOT_FOUND, Json(body)).into_response()
    }
}

async fn cancel_all_downloads(api: &SttApi) -> Response {
    let count = api.cancel_all_downloads().await;
    let body = CancelDownloadResponse {
        status: "cancelled",
        model_size: None,
        cancelled_count: count,
    };
    (StatusCode::OK, Json(body)).into_response()
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

/// List supported STT languages.
#[utoipa::path(
    get,
    path = "/languages",
    tag = "stt",
    responses(
        (status = 200, description = "List of supported language codes", body = Vec<String>),
    ),
)]
pub async fn list_languages() -> Json<Vec<&'static str>> {
    Json(SttApi::supported_languages().to_vec())
}

/// NDJSON event for download progress streaming.
#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
enum DownloadStreamEvent {
    #[serde(rename_all = "camelCase")]
    Progress {
        downloaded_bytes: u64,
        total_bytes: u64,
        percentage: f64,
    },
    #[serde(rename_all = "camelCase")]
    Complete {
        model_size: SttModelSize,
        path: String,
    },
    #[serde(rename_all = "camelCase")]
    Error { message: String },
}

fn serialize_download_event(event: &DownloadStreamEvent) -> Vec<u8> {
    let mut buf = serde_json::to_vec(event).unwrap_or_default();
    buf.push(b'\n');
    buf
}

fn relative_model_path(size: SttModelSize) -> String {
    let path = model_path(size);
    format!(
        "models/{}",
        path.file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default()
    )
}

/// Download an STT model with NDJSON progress streaming.
///
/// Streams progress events as NDJSON lines. If the model already exists,
/// returns a single `complete` event. Returns 409 if a download is already in progress.
#[utoipa::path(
    post,
    path = "/models/download/stream",
    tag = "stt",
    request_body = ModelDownloadRequest,
    responses(
        (status = 200, description = "NDJSON stream of download progress", content_type = "application/x-ndjson"),
        (status = 409, description = "Download already in progress"),
        (status = 422, description = "Invalid model size"),
    ),
)]
pub async fn download_model_stream(
    State(api): State<SttApi>,
    Json(body): Json<ModelDownloadRequest>,
) -> Response {
    let size = body.model_size;

    if api.is_model_available(size) {
        return ndjson_single_event(&DownloadStreamEvent::Complete {
            model_size: size,
            path: relative_model_path(size),
        });
    }

    if api.is_downloading(size).await {
        let body = serde_json::json!({
            "error": "download_in_progress",
            "message": "A download is already in progress for this model size",
        });
        return (StatusCode::CONFLICT, Json(body)).into_response();
    }

    let (rx, handle, _cancel) = api.start_download_stream(size).await;
    let (tx, mpsc_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(64);

    tokio::spawn(stream_download_progress(api, size, rx, handle, tx));

    ndjson_streaming_response(mpsc_rx)
}

/// Drive the download to completion, forwarding periodic progress events to `tx`.
async fn stream_download_progress(
    api: SttApi,
    size: SttModelSize,
    mut rx: tokio::sync::watch::Receiver<homunculus_microphone::DownloadProgress>,
    mut handle: tokio::task::JoinHandle<Result<(), homunculus_microphone::error::DownloadError>>,
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
) {
    let mut interval = tokio::time::interval(Duration::from_millis(250));
    interval.tick().await;

    let result = loop {
        tokio::select! {
            join_result = &mut handle => {
                break match join_result {
                    Ok(Ok(())) => Ok(()),
                    Ok(Err(e)) => Err(e.to_string()),
                    Err(e) => Err(e.to_string()),
                };
            }
            _ = interval.tick() => {
                let progress = rx.borrow_and_update().clone();
                let event = DownloadStreamEvent::Progress {
                    downloaded_bytes: progress.downloaded_bytes,
                    total_bytes: progress.total_bytes,
                    percentage: progress.percentage,
                };
                if tx.send(serialize_download_event(&event)).await.is_err() {
                    break Err("Client disconnected".to_string());
                }
            }
        }
    };

    api.finish_download(size).await;

    let final_event = match result {
        Ok(()) => DownloadStreamEvent::Complete {
            model_size: size,
            path: relative_model_path(size),
        },
        Err(message) => DownloadStreamEvent::Error { message },
    };
    let _ = tx.send(serialize_download_event(&final_event)).await;
}

/// Build a single-event NDJSON response (used when the model already exists).
fn ndjson_single_event(event: &DownloadStreamEvent) -> Response {
    let body = Body::from(serialize_download_event(event));
    Response::builder()
        .header("Content-Type", "application/x-ndjson")
        .header("Cache-Control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .unwrap()
        .into_response()
}

/// Build a streaming NDJSON response from an mpsc channel.
fn ndjson_streaming_response(mpsc_rx: tokio::sync::mpsc::Receiver<Vec<u8>>) -> Response {
    let stream = ReceiverStream::new(mpsc_rx);
    let body = Body::from_stream(stream.map(Ok::<_, Infallible>));
    Response::builder()
        .header("Content-Type", "application/x-ndjson")
        .header("Cache-Control", "no-store")
        .header("X-Content-Type-Options", "nosniff")
        .body(body)
        .unwrap()
        .into_response()
}

/// Wrapper for converting `SttError` into HTTP responses.
///
/// STT needs domain-specific HTTP status codes that don't map to the
/// shared `ApiError` variants.
pub struct SttErrorResponse(SttError);

impl From<SttError> for SttErrorResponse {
    fn from(err: SttError) -> Self {
        Self(err)
    }
}

impl IntoResponse for SttErrorResponse {
    fn into_response(self) -> Response {
        let (status, error_code) = error_to_status_code(&self.0);

        let body = serde_json::json!({
            "error": error_code,
            "message": self.0.to_string(),
        });

        (status, Json(body)).into_response()
    }
}

fn error_to_status_code(err: &SttError) -> (StatusCode, &'static str) {
    match err {
        SttError::ModelNotAvailable(_) => (StatusCode::SERVICE_UNAVAILABLE, "model_not_available"),
        SttError::ModelLoadFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "model_load_failed"),
        SttError::PipelineFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "pipeline_failed"),
        SttError::NoMicrophone => (StatusCode::SERVICE_UNAVAILABLE, "no_microphone"),
        SttError::MicrophonePermissionDenied => (
            StatusCode::SERVICE_UNAVAILABLE,
            "microphone_permission_denied",
        ),
        SttError::DownloadFailed(_) => (StatusCode::INTERNAL_SERVER_ERROR, "download_failed"),
        SttError::DownloadCancelled => (StatusCode::CONFLICT, "download_cancelled"),
        SttError::InvalidLanguage(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_language"),
        SttError::InvalidModelSize => (StatusCode::UNPROCESSABLE_ENTITY, "invalid_model_size"),
    }
}
