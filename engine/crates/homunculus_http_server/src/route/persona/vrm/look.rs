use axum::extract::{Path, State};
use bevy::prelude::Entity;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

use crate::route::persona::PersonaPath;

/// Set look-at to follow the cursor.
#[utoipa::path(
    put,
    path = "/vrm/look/cursor",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Look-at cursor mode set"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn look_cursor(State(api): State<VrmApi>, path: PersonaPath) -> HttpResult {
    api.look_at_cursor(path.entity).await.into_http_result()
}

/// Set look-at target to another entity.
#[utoipa::path(
    put,
    path = "/vrm/look/target/{target}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("target" = String, Path, description = "Target entity ID"),
    ),
    responses(
        (status = 200, description = "Look-at target set"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn look_target(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Path((_, target)): Path<(String, Entity)>,
) -> HttpResult {
    api.look_at_target(path.entity, target)
        .await
        .into_http_result()
}

/// Disable look-at control.
#[utoipa::path(
    delete,
    path = "/vrm/look",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Look-at disabled"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn unlook(State(api): State<VrmApi>, path: PersonaPath) -> HttpResult {
    api.unlook(path.entity).await.into_http_result()
}
