use crate::route::vrm::get::VrmGetQuery;
use axum::extract::{Query, State};
use homunculus_api::prelude::VrmApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Wait for a VRM model to load.
///
/// ### Path
///
/// `GET /vrm/wait-load`
///
/// ### Query
///
/// - `name`: The name of the VRM model.
pub async fn wait_load(
    State(api): State<VrmApi>,
    Query(query): Query<VrmGetQuery>,
) -> HttpResult<u64> {
    api.wait_load_by_name(query.name)
        .await
        .map(|e| e.to_bits())
        .into_http_result()
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
        assert_response(&mut app, router, request, entity.to_bits()).await;
    }
}
