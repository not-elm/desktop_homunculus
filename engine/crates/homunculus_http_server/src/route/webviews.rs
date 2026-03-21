use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use bevy::prelude::Entity;
use homunculus_api::prelude::WebviewApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;

/// Open a global webview in world space.
#[utoipa::path(
    post,
    path = "/",
    tag = "webviews",
    request_body = WebviewOpenOptions,
    responses(
        (status = 200, description = "Webview opened, returns entity ID", body = String),
    ),
)]
pub async fn open(
    State(api): State<WebviewApi>,
    Json(options): Json<WebviewOpenOptions>,
) -> HttpResult<Entity> {
    api.open(options).await.into_http_result()
}

/// Check if a webview is closed.
#[utoipa::path(
    get,
    path = "/{entity}/is-closed",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Webview closed status", body = bool),
    ),
)]
pub async fn is_closed(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult<bool> {
    api.is_closed(entity).await.into_http_result()
}

/// Request body for setting a linked character.
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetLinkedCharacterRequest {
    /// The character ID to link.
    pub character_id: String,
}

/// Get the linked character for a webview.
#[utoipa::path(
    get,
    path = "/{entity}/linked-character",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Linked character ID", body = Option<String>),
    ),
)]
pub async fn get_linked_character(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult<Option<String>> {
    api.linked_character(entity).await.into_http_result()
}

/// Set the linked character for a webview.
#[utoipa::path(
    put,
    path = "/{entity}/linked-character",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = SetLinkedCharacterRequest,
    responses(
        (status = 200, description = "Character linked to webview"),
    ),
)]
pub async fn set_linked_character(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
    Json(body): Json<SetLinkedCharacterRequest>,
) -> HttpResult {
    api.set_linked_character(entity, body.character_id)
        .await
        .into_http_result()
}

/// Remove the linked character from a webview.
#[utoipa::path(
    delete,
    path = "/{entity}/linked-character",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Character unlinked from webview"),
    ),
)]
pub async fn unlink_character(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult {
    api.unlink_character(entity).await.into_http_result()
}

// --- Deprecated linked-vrm routes (backward compatibility) ---

/// Deprecated request body for setting a linked VRM.
#[derive(Deserialize, ToSchema)]
pub struct SetLinkedVrmRequest {
    #[schema(value_type = String)]
    pub vrm: Entity,
}

/// Get the linked VRM entity for a webview (deprecated).
#[utoipa::path(
    get,
    path = "/{entity}/linked-vrm",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Linked VRM entity ID (deprecated)", body = Option<String>),
    ),
)]
pub async fn get_linked_vrm(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult<Option<Entity>> {
    api.linked_character_entity(entity).await.into_http_result()
}

/// Set the linked VRM for a webview (deprecated, prefer linked-character).
#[utoipa::path(
    put,
    path = "/{entity}/linked-vrm",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = SetLinkedVrmRequest,
    responses(
        (status = 200, description = "VRM linked to webview (deprecated)"),
    ),
)]
pub async fn set_linked_vrm(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
    Json(body): Json<SetLinkedVrmRequest>,
) -> HttpResult {
    // Resolve the Entity to a character ID via CharacterRegistry, then store the character ID
    api.set_linked_character_by_entity(entity, body.vrm)
        .await
        .into_http_result()
}

/// Remove the linked VRM from a webview (deprecated).
#[utoipa::path(
    delete,
    path = "/{entity}/linked-vrm",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "VRM unlinked from webview (deprecated)"),
    ),
)]
pub async fn unlink_vrm(State(api): State<WebviewApi>, EntityId(entity): EntityId) -> HttpResult {
    api.unlink_character(entity).await.into_http_result()
}

/// List all open webviews.
#[utoipa::path(
    get,
    path = "/",
    tag = "webviews",
    responses(
        (status = 200, description = "List of open webviews", body = Vec<WebviewInfo>),
    ),
)]
pub async fn list(State(api): State<WebviewApi>) -> HttpResult<Vec<WebviewInfo>> {
    api.list().await.into_http_result()
}

/// Get detailed info for a specific webview.
#[utoipa::path(
    get,
    path = "/{entity}",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Webview info", body = WebviewInfo),
        (status = 404, description = "Webview not found"),
    ),
)]
pub async fn get(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult<WebviewInfo> {
    api.get(entity).await.into_http_result()
}

/// Partial update of a webview.
#[utoipa::path(
    patch,
    path = "/{entity}",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = WebviewPatchRequest,
    responses(
        (status = 200, description = "Webview updated"),
        (status = 404, description = "Webview not found"),
    ),
)]
pub async fn patch(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
    Json(body): Json<WebviewPatchRequest>,
) -> HttpResult {
    api.patch(entity, body).await.into_http_result()
}

/// Navigate back in history.
#[utoipa::path(
    post,
    path = "/{entity}/navigate/back",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Navigated back"),
    ),
)]
pub async fn navigate_back(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult {
    api.go_back(entity).await.into_http_result()
}

/// Navigate forward in history.
#[utoipa::path(
    post,
    path = "/{entity}/navigate/forward",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Navigated forward"),
    ),
)]
pub async fn navigate_forward(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult {
    api.go_forward(entity).await.into_http_result()
}

/// Navigate to a new URL.
#[utoipa::path(
    post,
    path = "/{entity}/navigate",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = WebviewNavigateRequest,
    responses(
        (status = 200, description = "Navigation initiated"),
    ),
)]
pub async fn navigate(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
    Json(body): Json<WebviewNavigateRequest>,
) -> HttpResult {
    api.navigate(entity, body.source).await.into_http_result()
}

/// Reload the current page.
#[utoipa::path(
    post,
    path = "/{entity}/reload",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Page reloaded"),
    ),
)]
pub async fn reload(State(api): State<WebviewApi>, EntityId(entity): EntityId) -> HttpResult {
    api.reload(entity).await.into_http_result()
}

/// Delete (close) a webview.
#[utoipa::path(
    delete,
    path = "/{entity}",
    tag = "webviews",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "Webview closed"),
        (status = 404, description = "Webview not found"),
    ),
)]
pub async fn delete(State(api): State<WebviewApi>, EntityId(entity): EntityId) -> HttpResult {
    api.close(entity).await.into_http_result()
}
