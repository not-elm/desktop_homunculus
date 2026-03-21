use crate::extract::character::VrmGuard;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

/// Despawn a VRM model from a character.
#[utoipa::path(
    delete,
    path = "/despawn",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "VRM model despawned"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn despawn(State(api): State<VrmApi>, VrmGuard { entity, .. }: VrmGuard) -> HttpResult {
    api.despawn(entity).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{call, spawn_character_with_vrm, test_app};
    use bevy_vrm1::vrm::Vrm;

    #[tokio::test]
    async fn test_despawn_vrm() {
        let (mut app, router) = test_app();
        spawn_character_with_vrm(&mut app, "test-char");

        let request = axum::http::Request::delete("/characters/test-char/vrm/despawn")
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert!(app.world_mut().query::<&Vrm>().single(app.world()).is_err());
    }
}
