use crate::extract::character::VrmGuard;
use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::character::CharacterApi;
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::CharacterId;

/// Disable look-at control for the specified character's VRM.
#[utoipa::path(
    delete,
    path = "/look",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Look-at disabled"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn unlook(State(api): State<VrmApi>, VrmGuard { entity, .. }: VrmGuard) -> HttpResult {
    api.unlook(entity).await.into_http_result()
}

/// Set look-at target to another entity.
#[utoipa::path(
    put,
    path = "/look/target/{target}",
    tag = "vrm",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("target" = String, Path, description = "Target entity ID"),
    ),
    responses(
        (status = 200, description = "Look-at target set"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn target(
    State(vrm_api): State<VrmApi>,
    State(char_api): State<CharacterApi>,
    Path((id_str, target)): Path<(String, Entity)>,
) -> HttpResult {
    let id = CharacterId::new(&id_str).map_err(|e| ApiError::InvalidCharacterId(e.to_string()))?;
    let entity = char_api.resolve_with_vrm(id).await?;
    vrm_api
        .look_at_target(entity, target)
        .await
        .into_http_result()
}

/// Set look-at to follow the cursor.
#[utoipa::path(
    put,
    path = "/look/cursor",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Look-at cursor mode set"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn cursor(State(api): State<VrmApi>, VrmGuard { entity, .. }: VrmGuard) -> HttpResult {
    api.look_at_cursor(entity).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{call, spawn_character_with_vrm, test_app};
    use bevy::prelude::*;
    use bevy_vrm1::prelude::LookAt;

    #[tokio::test]
    async fn test_unlook() {
        let (mut app, router) = test_app();
        let entity = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(entity).insert(LookAt::Cursor);

        let request = axum::http::Request::delete("/characters/test-char/vrm/look")
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert!(
            !app.world().entity(entity).contains::<LookAt>(),
            "LookAt component should be removed"
        );
    }

    #[tokio::test]
    async fn test_look_at_target() {
        let (mut app, router) = test_app();
        let entity = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(entity).insert(LookAt::Cursor);
        let target = app.world_mut().spawn_empty().id();

        let request = axum::http::Request::put(format!(
            "/characters/test-char/vrm/look/target/{}",
            target.to_bits()
        ))
        .body(axum::body::Body::empty())
        .unwrap();
        call(&mut app, router, request).await;
        assert_eq!(
            app.world().entity(entity).get::<LookAt>().unwrap(),
            &LookAt::Target(target),
        );
    }

    #[tokio::test]
    async fn test_look_at_cursor() {
        let (mut app, router) = test_app();
        let entity = spawn_character_with_vrm(&mut app, "test-char");
        app.world_mut().entity_mut(entity).insert(LookAt::Cursor);

        let request = axum::http::Request::put("/characters/test-char/vrm/look/cursor")
            .body(axum::body::Body::empty())
            .unwrap();
        call(&mut app, router, request).await;
        assert_eq!(
            app.world().entity(entity).get::<LookAt>().unwrap(),
            &LookAt::Cursor,
        );
    }
}
