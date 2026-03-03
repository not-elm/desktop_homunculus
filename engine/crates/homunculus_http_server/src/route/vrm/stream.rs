use async_channel::Receiver;
use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use futures::Stream;
use futures::stream::unfold;
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::VrmMetadata;
use std::convert::Infallible;
use std::time::Duration;

/// Stream VRM model load events via SSE.
///
/// Returns an SSE stream that emits an event each time a VRM model is loaded.
#[utoipa::path(
    get,
    path = "/stream",
    tag = "vrm",
    responses(
        (status = 200, description = "SSE stream of VRM load events", content_type = "text/event-stream"),
    ),
)]
pub async fn stream(
    State(api): State<VrmApi>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, axum::response::Response> {
    let rx = api.observer().await.map_err(|e| e.into_response())?;
    Ok(Sse::new(to_stream(rx)).keep_alive(KeepAlive::new().interval(Duration::from_secs(30))))
}

use axum::response::IntoResponse;

fn to_stream(rx: Receiver<VrmMetadata>) -> impl Stream<Item = Result<Event, Infallible>> {
    unfold(rx, |rx| async move {
        match rx.recv().await {
            Ok(metadata) => {
                let data = serde_json::to_string(&metadata).unwrap_or_default();
                let event = Event::default().data(data);
                Some((Ok(event), rx))
            }
            Err(_) => None,
        }
    })
}
