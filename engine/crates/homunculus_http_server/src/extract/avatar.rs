use axum::Json;
use axum::extract::{FromRef, FromRequestParts, Path};
use axum::http::StatusCode;
use axum::http::request::Parts;
use bevy::prelude::Entity;
use homunculus_api::avatar::AvatarApi;
use homunculus_core::prelude::AvatarId;

/// Extracts and resolves an avatar ID from the URL path.
///
/// Validates the raw path parameter via [`AvatarId::new()`], then resolves
/// the ID to an ECS [`Entity`] through [`AvatarApi::resolve()`].
///
/// The `entity` field is available for handlers that need direct ECS access
/// (e.g. VRM sub-routes in a later migration phase).
pub struct AvatarIdExtractor {
    pub id: AvatarId,
    #[allow(dead_code)]
    pub entity: Entity,
}

impl<S> FromRequestParts<S> for AvatarIdExtractor
where
    AvatarApi: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id_str) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(bad_path)?;

        let id = AvatarId::new(&id_str).map_err(bad_avatar_id)?;

        let api = AvatarApi::from_ref(state);
        let entity = api.resolve(id.clone()).await.map_err(not_found)?;

        Ok(Self { id, entity })
    }
}

/// Extracts an avatar ID and verifies a VRM is attached.
///
/// Uses [`AvatarApi::resolve_with_vrm()`] which returns an error when
/// the avatar exists but has no VRM model loaded.
///
/// Will be used when existing `/vrm/{entity}/...` sub-routes are migrated
/// to `/avatars/{id}/vrm/...`.
#[allow(dead_code)]
pub struct VrmGuard {
    pub id: AvatarId,
    pub entity: Entity,
}

impl<S> FromRequestParts<S> for VrmGuard
where
    AvatarApi: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id_str) = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(bad_path)?;

        let id = AvatarId::new(&id_str).map_err(bad_avatar_id)?;

        let api = AvatarApi::from_ref(state);
        let entity = api.resolve_with_vrm(id.clone()).await.map_err(not_found)?;

        Ok(Self { id, entity })
    }
}

fn bad_path<E: std::fmt::Display>(e: E) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": e.to_string()})),
    )
}

fn bad_avatar_id<E: std::fmt::Display>(e: E) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": e.to_string()})),
    )
}

fn not_found<E: std::fmt::Display>(e: E) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": e.to_string()})),
    )
}
