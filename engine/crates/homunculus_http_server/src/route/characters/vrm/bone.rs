use axum::extract::{Path, State};
use bevy::prelude::Entity;
use bevy_vrm1::vrm::VrmBone;
use homunculus_api::prelude::VrmApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::CharacterId;

/// Get the entity ID of a specific bone in a VRM model.
#[utoipa::path(
    get,
    path = "/{bone_name}",
    tag = "vrm",
    params(
        ("id" = String, Path, description = "Character ID"),
        ("bone_name" = String, Path, description = "Bone name"),
    ),
    responses(
        (status = 200, description = "Bone entity ID", body = String),
        (status = 404, description = "VRM or bone not found"),
    ),
)]
pub async fn get(
    State(api): State<VrmApi>,
    Path((id, bone_name)): Path<(CharacterId, String)>,
) -> HttpResult<Entity> {
    let bone_name = VrmBone(bone_name);
    api.bone(id, bone_name).await.into_http_result()
}
