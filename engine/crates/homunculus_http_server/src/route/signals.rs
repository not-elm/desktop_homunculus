//! `/signals` provides a mechanism for bridging between external processes.
//! For example, it can be used to send values from external applications created by users to a Webview created within a MOD.

use axum::Json;
use axum::extract::{Path, State};
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use bevy::tasks::futures_lite::{Stream, StreamExt};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{ApiError, SignalInfo, SignalsApi};
use std::convert::Infallible;
use std::time::Duration;

/// List all active signal channels.
#[utoipa::path(
    get,
    path = "/",
    tag = "signals",
    responses(
        (status = 200, description = "List of active signal channels", body = Vec<SignalInfo>),
    ),
)]
pub async fn list(State(api): State<SignalsApi>) -> HttpResult<Vec<SignalInfo>> {
    api.list().await.into_http_result()
}

/// Stream events for a specific signal via SSE.
///
/// The signals are sent via `POST /signals/{signal}` API.
#[utoipa::path(
    get,
    path = "/{signal}",
    tag = "signals",
    params(
        ("signal" = String, Path, description = "Signal channel name"),
    ),
    responses(
        (status = 200, description = "SSE event stream", content_type = "text/event-stream"),
    ),
)]
pub async fn stream(
    State(api): State<SignalsApi>,
    Path(signal): Path<String>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send + Sync>, ApiError> {
    let stream = api.clone().stream(signal).await?;
    let stream = stream.map(|value| {
        let event = Event::default().data(serde_json::to_string(&value).unwrap());
        Ok(event)
    });
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

/// Send a signal to all subscribers.
///
/// The signal is sent to all processes that are streaming the signal at `GET /signals/{signal}`.
#[utoipa::path(
    post,
    path = "/{signal}",
    tag = "signals",
    params(
        ("signal" = String, Path, description = "Signal channel name"),
    ),
    request_body = Object,
    responses(
        (status = 200, description = "Signal sent"),
    ),
)]
pub async fn send(
    State(api): State<SignalsApi>,
    Path(signal): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<(), ApiError> {
    api.send(signal, body).await?;
    Ok(())
}
