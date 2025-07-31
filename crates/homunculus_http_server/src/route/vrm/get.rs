use axum::extract::{Query, State};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VrmGetQuery {
    pub name: String,
}

/// Find a VRM model by name.
///
/// ### Path
///
/// `GET /vrm`
///
/// ### Query
///
/// - `name`: The name of the VRM model.
pub async fn get(State(api): State<VrmApi>, Query(query): Query<VrmGetQuery>) -> HttpResult<u64> {
    api.find_by_name(query.name)
        .await
        .map(|e| e.to_bits())
        .into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::vrm::Vrm;

    #[tokio::test]
    async fn test_fetch_entity() {
        let (mut app, router) = test_app();

        let entity = app.world_mut().spawn((Name::new("Test"), Vrm)).id();
        app.update();

        let request = axum::http::Request::get("/vrm?name=Test".to_string())
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, entity.to_bits()).await;
    }
}
