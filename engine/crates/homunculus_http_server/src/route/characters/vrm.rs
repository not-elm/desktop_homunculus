use axum::{Json, extract::State};
use homunculus_api::prelude::{
    CharacterApi,
    axum::{HttpResult, IntoHttpResult},
};
use homunculus_core::prelude::AssetId;

use crate::{extract::character::CharacterIdExtractor, route::characters::AttachVrmBody};

/// Attach a VRM model to a character.
#[utoipa::path(
    post,
    path = "/vrm/attach",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    request_body = AttachVrmBody,
    responses(
        (status = 200, description = "VRM attached"),
        (status = 404, description = "Character not found"),
    ),
)]
pub async fn attach_vrm(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
    Json(body): Json<AttachVrmBody>,
) -> HttpResult {
    let asset_id = AssetId::new(&body.asset_id);
    api.attach_vrm(id, asset_id).await?;
    Ok(Json(()))
}

/// Detach the VRM model from a character.
#[utoipa::path(
    delete,
    path = "/vrm",
    tag = "characters",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "VRM detached"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn detach_vrm(
    State(api): State<CharacterApi>,
    CharacterIdExtractor { id, .. }: CharacterIdExtractor,
) -> HttpResult {
    api.detach_vrm(id).await.into_http_result()
}
