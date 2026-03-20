use axum::Json;
use axum::extract::{Path, State};
use homunculus_api::avatar::AvatarApi;
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::AvatarId;

/// Get extension data for a specific mod on an avatar.
#[utoipa::path(
    get,
    path = "",
    tag = "avatars",
    params(
        ("id" = String, Path, description = "Avatar ID"),
        ("mod_name" = String, Path, description = "Mod name"),
    ),
    responses(
        (status = 200, description = "Extension data", body = serde_json::Value),
        (status = 404, description = "Avatar or extension not found"),
    ),
)]
pub async fn get_extension(
    Path((id_str, mod_name)): Path<(String, String)>,
    State(api): State<AvatarApi>,
) -> HttpResult<serde_json::Value> {
    let id = parse_avatar_id(&id_str)?;
    api.get_extension(id, mod_name).await.into_http_result()
}

/// Set extension data for a specific mod on an avatar.
#[utoipa::path(
    put,
    path = "",
    tag = "avatars",
    params(
        ("id" = String, Path, description = "Avatar ID"),
        ("mod_name" = String, Path, description = "Mod name"),
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Extension data updated"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn set_extension(
    Path((id_str, mod_name)): Path<(String, String)>,
    State(api): State<AvatarApi>,
    Json(data): Json<serde_json::Value>,
) -> HttpResult {
    let id = parse_avatar_id(&id_str)?;
    api.set_extension(id, mod_name, data)
        .await
        .into_http_result()
}

/// Delete extension data for a specific mod on an avatar.
#[utoipa::path(
    delete,
    path = "",
    tag = "avatars",
    params(
        ("id" = String, Path, description = "Avatar ID"),
        ("mod_name" = String, Path, description = "Mod name"),
    ),
    responses(
        (status = 200, description = "Extension data deleted"),
        (status = 404, description = "Avatar not found"),
    ),
)]
pub async fn delete_extension(
    Path((id_str, mod_name)): Path<(String, String)>,
    State(api): State<AvatarApi>,
) -> HttpResult {
    let id = parse_avatar_id(&id_str)?;
    api.delete_extension(id, mod_name)
        .await
        .into_http_result()
}

/// Parses and validates a raw avatar ID string.
fn parse_avatar_id(raw: &str) -> Result<AvatarId, ApiError> {
    AvatarId::new(raw).map_err(|e| ApiError::InvalidAvatarId(e.to_string()))
}
