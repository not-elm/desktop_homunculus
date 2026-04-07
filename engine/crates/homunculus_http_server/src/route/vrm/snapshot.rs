use super::get::VrmGetQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use homunculus_api::prelude::axum::IntoHttpResult;
use homunculus_api::vrm::{VrmApi, VrmSnapshot};

/// Get detailed snapshot of all VRM instances.
#[utoipa::path(
    get,
    path = "/snapshot",
    tag = "vrm",
    params(
        ("name" = Option<String>, Query, description = "Optional name filter"),
    ),
    responses(
        (status = 200, description = "VRM snapshots", body = Vec<VrmSnapshot>),
    ),
)]
pub async fn snapshot(State(api): State<VrmApi>, Query(query): Query<VrmGetQuery>) -> Response {
    let result = api.snapshot().await;
    match query.name {
        Some(name) => result
            .map(|all| {
                all.into_iter()
                    .filter(|s| s.name == name)
                    .collect::<Vec<VrmSnapshot>>()
            })
            .into_http_result()
            .into_response(),
        None => result.into_http_result().into_response(),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_app;
    use bevy::prelude::*;
    use bevy_vrm1::vrm::{Initialized, Vrm};
    use homunculus_api::vrm::VrmSnapshot;
    use homunculus_core::prelude::{AssetId, AssetIdComponent, PersonaState};

    #[tokio::test]
    async fn test_snapshot() {
        let (mut app, router) = test_app();

        app.world_mut().spawn((
            Name::new("Test1"),
            Vrm,
            Initialized,
            PersonaState::default(),
            Transform::default(),
            AssetIdComponent(AssetId::new("test-mod:test-vrm")),
        ));
        app.update();

        let request = axum::http::Request::get("/vrm/snapshot")
            .body(axum::body::Body::empty())
            .unwrap();
        let response = crate::tests::call(&mut app, router, request).await;
        let body = http_body_util::BodyExt::collect(response.into_body())
            .await
            .unwrap()
            .to_bytes();
        let snapshots: Vec<VrmSnapshot> = serde_json::from_slice(&body).unwrap();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].name, "Test1");
        assert_eq!(snapshots[0].state, "idle");
        assert_eq!(snapshots[0].asset_id, Some("test-mod:test-vrm".to_string()));
    }

    #[tokio::test]
    async fn test_snapshot_with_name_filter() {
        let (mut app, router) = test_app();

        app.world_mut().spawn((
            Name::new("Test1"),
            Vrm,
            Initialized,
            PersonaState::default(),
            Transform::default(),
        ));
        app.world_mut().spawn((
            Name::new("Test2"),
            Vrm,
            Initialized,
            PersonaState::default(),
            Transform::default(),
        ));
        app.update();

        let request = axum::http::Request::get("/vrm/snapshot?name=Test1")
            .body(axum::body::Body::empty())
            .unwrap();
        let response = crate::tests::call(&mut app, router, request).await;
        let body = http_body_util::BodyExt::collect(response.into_body())
            .await
            .unwrap()
            .to_bytes();
        let snapshots: Vec<VrmSnapshot> = serde_json::from_slice(&body).unwrap();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].name, "Test1");
    }
}
