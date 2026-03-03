use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::IntoHttpResult;
use homunculus_api::vrm::VrmApi;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, ToSchema)]
pub struct VrmGetQuery {
    pub name: Option<String>,
}

/// List VRM model entities, optionally filtered by name.
#[utoipa::path(
    get,
    path = "/",
    tag = "vrm",
    params(
        ("name" = Option<String>, Query, description = "Optional name filter"),
    ),
    responses(
        (status = 200, description = "List of VRM entity IDs", body = Vec<String>),
    ),
)]
pub async fn get(State(api): State<VrmApi>, Query(query): Query<VrmGetQuery>) -> Response {
    let result = api.fetch_all().await;
    match query.name {
        Some(name) => result
            .map(|all| {
                all.into_iter()
                    .filter(|(_, n)| *n == name)
                    .map(|(e, _)| e)
                    .collect::<Vec<Entity>>()
            })
            .into_http_result()
            .into_response(),
        None => result
            .map(|all| all.into_iter().map(|(e, _)| e).collect::<Vec<Entity>>())
            .into_http_result()
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::vrm::{Initialized, Vrm};

    #[tokio::test]
    async fn test_fetch_all() {
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

        let request = axum::http::Request::get("/vrm")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, vec![entity1, entity2]).await;
    }

    #[tokio::test]
    async fn test_fetch_by_name() {
        let (mut app, router) = test_app();

        let entity = app
            .world_mut()
            .spawn((Name::new("Test"), Vrm, Initialized))
            .id();
        app.world_mut()
            .spawn((Name::new("Other"), Vrm, Initialized));
        app.update();

        let request = axum::http::Request::get("/vrm?name=Test")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, vec![entity]).await;
    }
}
