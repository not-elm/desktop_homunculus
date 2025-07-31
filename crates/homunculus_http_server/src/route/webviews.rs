use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use bevy::prelude::Entity;
use homunculus_api::prelude::WebviewApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::*;

/// Open a webview.
///
/// ### Path
///
/// `POST /webview/`
pub async fn open(
    State(api): State<WebviewApi>,
    Json(options): Json<WebviewOpenOptions>,
) -> HttpResult<Entity> {
    api.open(options).await.into_http_result()
}

/// Close a webview.
///
/// ### Path
///
/// `DELETE /webview/:entity_id/close`
pub async fn close(State(api): State<WebviewApi>, EntityId(entity): EntityId) -> HttpResult {
    api.close(entity).await.into_http_result()
}

/// Check if a webview is closed.
///
/// ### Path
///
/// `GET /webview/:entity_id/is-closed`
pub async fn is_closed(
    State(api): State<WebviewApi>,
    EntityId(entity): EntityId,
) -> HttpResult<bool> {
    api.is_closed(entity).await.into_http_result()
}
