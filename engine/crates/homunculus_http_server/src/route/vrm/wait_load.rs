use axum::extract::{Query, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::VrmApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct VrmNameQuery {
    pub name: String,
}

/// Wait for a VRM model to load.
#[utoipa::path(
    get,
    path = "/wait-load",
    tag = "vrm",
    params(
        ("name" = String, Query, description = "Name of the VRM model to wait for"),
    ),
    responses(
        (status = 200, description = "VRM loaded, returns entity ID", body = String),
    ),
)]
pub async fn wait_load(
    State(api): State<VrmApi>,
    Query(query): Query<VrmNameQuery>,
) -> HttpResult<Entity> {
    api.wait_load_by_name(query.name).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::vrm::{Initialized, Vrm};

    #[tokio::test]
    async fn test_fetch_entity() {
        let (mut app, router) = test_app();

        let entity = app
            .world_mut()
            .spawn((Name::new("Test"), Vrm, Initialized))
            .id();
        app.update();

        let request = axum::http::Request::get("/vrm/wait-load?name=Test".to_string())
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, entity).await;
    }
}
