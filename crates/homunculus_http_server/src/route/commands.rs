//! `/commands` provides a mechanism for bridging between external processes.
//! For example, it can be used to send values from external applications created by users to a Webview created within a MOD.

use axum::Json;
use axum::extract::{Path, State};
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use bevy::tasks::futures_lite::{Stream, StreamExt};
use homunculus_api::prelude::{ApiError, CommandsApi};
use std::convert::Infallible;
use std::time::Duration;

/// Handles the `GET /commands/{command}` route to stream events for a specific command.
///
/// The commands are sent via `POST commands/{command}` API.
pub async fn stream(
    State(api): State<CommandsApi>,
    Path(command): Path<String>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + Send + Sync>, ApiError> {
    let stream = api.clone().stream(command).await?;
    let stream = stream.map(|value| {
        let event = Event::default().data(serde_json::to_string(&value).unwrap());
        Ok(event)
    });
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

/// Handles the `POST /commands/{command}` route to send a command.
///
/// The command is sent to all processes that are streaming the command at `GET /commands/{command}`.
pub async fn send(
    State(api): State<CommandsApi>,
    Path(command): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<(), ApiError> {
    api.send(command, body).await?;
    Ok(())
}
