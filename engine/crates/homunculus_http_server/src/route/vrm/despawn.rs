use crate::extract::EntityId;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

/// Despawn a VRM model.
#[utoipa::path(
    delete,
    path = "/",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "VRM model despawned"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn despawn(State(api): State<VrmApi>, EntityId(entity): EntityId) -> HttpResult {
    api.despawn(entity).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{call, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::vrm::Vrm;

    #[tokio::test]
    async fn test_despawn_vrm() {
        let (mut app, router) = test_app();

        let entity = app.world_mut().spawn((Name::new("Test"), Vrm)).id();
        app.update();

        let request = axum::http::Request::delete(format!("/vrm/{}", entity.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert!(app.world_mut().query::<&Vrm>().single(app.world()).is_err());
    }
}
