use axum::Json;
use axum::extract::{Path, Query, State};
use homunculus_api::character::CharacterApi;
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::CharacterId;
use serde::Deserialize;

/// Query parameters for extension endpoints.
#[derive(Deserialize)]
pub struct ExtensionQuery {
    #[serde(rename = "mod")]
    pub mod_name: String,
}

/// Get extension data for a specific mod on a character.
#[utoipa::path(
    get,
    path = "",
    tag = "characters",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("mod" = String, Query, description = "Mod package name (e.g. @hmcs/voicevox)"),
    ),
    responses(
        (status = 200, description = "Extension data", body = serde_json::Value),
        (status = 404, description = "Character or extension not found"),
    ),
)]
pub async fn get_extension(
    Path(id_str): Path<String>,
    Query(query): Query<ExtensionQuery>,
    State(api): State<CharacterApi>,
) -> HttpResult<serde_json::Value> {
    let id = parse_character_id(&id_str)?;
    api.get_extension(id, query.mod_name)
        .await
        .into_http_result()
}

/// Set extension data for a specific mod on a character.
#[utoipa::path(
    put,
    path = "",
    tag = "characters",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("mod" = String, Query, description = "Mod package name (e.g. @hmcs/voicevox)"),
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Extension data updated"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn set_extension(
    Path(id_str): Path<String>,
    Query(query): Query<ExtensionQuery>,
    State(api): State<CharacterApi>,
    Json(data): Json<serde_json::Value>,
) -> HttpResult {
    let id = parse_character_id(&id_str)?;
    api.set_extension(id, query.mod_name, data)
        .await
        .into_http_result()
}

/// Delete extension data for a specific mod on a character.
#[utoipa::path(
    delete,
    path = "",
    tag = "characters",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("mod" = String, Query, description = "Mod package name (e.g. @hmcs/voicevox)"),
    ),
    responses(
        (status = 200, description = "Extension data deleted"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn delete_extension(
    Path(id_str): Path<String>,
    Query(query): Query<ExtensionQuery>,
    State(api): State<CharacterApi>,
) -> HttpResult {
    let id = parse_character_id(&id_str)?;
    api.delete_extension(id, query.mod_name)
        .await
        .into_http_result()
}

/// Parses and validates a raw character ID string.
fn parse_character_id(raw: &str) -> Result<CharacterId, ApiError> {
    CharacterId::new(raw).map_err(|e| ApiError::InvalidCharacterId(e.to_string()))
}
