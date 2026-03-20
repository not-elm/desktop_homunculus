use crate::extract::EntityId;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{SetNameRequest, VrmApi, VrmNames};

/// Query parameters for VRM name operations.
#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct VrmNameQuery {
    /// BCP-47 language code (e.g. "en", "ja"). Defaults to "en".
    #[serde(default = "default_language")]
    pub lang: String,
}

fn default_language() -> String {
    "en".to_string()
}

/// Get the display name of a VRM entity for a given language.
#[utoipa::path(
    get,
    path = "/name",
    tag = "vrm",
    params(
        ("entity" = String, Path, description = "Entity ID"),
        VrmNameQuery,
    ),
    responses(
        (status = 200, description = "VRM display name", body = String),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn get(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
    axum::extract::Query(query): axum::extract::Query<VrmNameQuery>,
) -> HttpResult<String> {
    api.get_name(entity, query.lang).await.into_http_result()
}

/// Set the display name of a VRM entity for a given language.
#[utoipa::path(
    put,
    path = "/name",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    request_body = SetNameRequest,
    responses(
        (status = 200, description = "Name updated"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn put(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
    axum::extract::Json(body): axum::extract::Json<SetNameRequest>,
) -> HttpResult {
    api.set_name(entity, body.language, body.name)
        .await
        .into_http_result()
}

/// Delete the display name of a VRM entity for a given language.
#[utoipa::path(
    delete,
    path = "/name",
    tag = "vrm",
    params(
        ("entity" = String, Path, description = "Entity ID"),
        VrmNameQuery,
    ),
    responses(
        (status = 200, description = "Name deleted"),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn delete(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
    axum::extract::Query(query): axum::extract::Query<VrmNameQuery>,
) -> HttpResult {
    api.delete_name(entity, query.lang)
        .await
        .into_http_result()
}

/// List all multilingual display names of a VRM entity.
#[utoipa::path(
    get,
    path = "/names",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "All VRM names", body = VrmNames),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn list(
    axum::extract::State(api): axum::extract::State<VrmApi>,
    EntityId(entity): EntityId,
) -> HttpResult<VrmNames> {
    api.list_names(entity).await.into_http_result()
}
