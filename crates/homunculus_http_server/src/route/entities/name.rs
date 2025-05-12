use crate::extract::EntityId;
use axum::extract::State;
use homunculus_api::prelude::EntitiesApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Get the entity name.
///
/// ### Path
///
/// `GET /entities/:entity_id/name`
pub async fn get(State(api): State<EntitiesApi>, EntityId(entity): EntityId) -> HttpResult<String> {
    api.name(entity).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, test_app};
    use bevy::prelude::Name;

    #[tokio::test]
    async fn get_name() {
        let (mut app, router) = test_app();

        let entity = app.world_mut().spawn(Name::new("VRM")).id();
        app.update();
        let request = axum::http::Request::get(format!("/entities/{}/name", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, "VRM".to_string()).await;
    }
}
