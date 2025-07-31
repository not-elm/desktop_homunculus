use async_channel::Receiver;
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive};
use axum::response::{IntoResponse, Response, Sse};
use futures::Stream;
use futures::stream::unfold;
use homunculus_api::prelude::axum::IntoHttpResult;
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::VrmMetadata;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::time::Duration;

/// Fetch all VRM models.
///
/// ### Path
///
/// `GET /vrm/all`
pub async fn all(State(api): State<VrmApi>, Query(query): Query<VrmAllQuery>) -> Response {
    if query.stream.is_some_and(|stream| stream) {
        match api.observer().await {
            Ok(rx) => Sse::new(to_stream(rx))
                .keep_alive(KeepAlive::new().interval(Duration::from_secs(30)))
                .into_response(),
            Err(e) => e.into_response(),
        }
    } else {
        api.fetch_all().await.into_http_result().into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VrmAllQuery {
    /// If true, the response will be in SSE stream format.
    /// In that case, an event will be sent every time a VRM is loaded.
    pub stream: Option<bool>,
}

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

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::vrm::{Initialized, Vrm};
    use homunculus_core::prelude::VrmMetadata;

    #[tokio::test]
    async fn test_fetch_entity() {
        let (mut app, router) = test_app();

        let entity1 = app
            .world_mut()
            .spawn((Name::new("Test1"), Vrm, Initialized))
            .id();
        let entity2 = app
            .world_mut()
            .spawn((Name::new("Test2"), Vrm, Initialized))
            .id();
        app.update();

        let request = axum::http::Request::get("/vrm/all".to_string())
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(
            &mut app,
            router,
            request,
            vec![
                VrmMetadata {
                    entity: entity1,
                    name: "Test1".to_string(),
                },
                VrmMetadata {
                    entity: entity2,
                    name: "Test2".to_string(),
                },
            ],
        )
        .await;
    }
}
