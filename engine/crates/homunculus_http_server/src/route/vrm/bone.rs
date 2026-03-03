use axum::extract::{Path, State};
use bevy::prelude::Entity;
use bevy_vrm1::vrm::VrmBone;
use homunculus_api::prelude::VrmApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Get the entity ID of a specific bone in a VRM model.
#[utoipa::path(
    get,
    path = "/{bone_name}",
    tag = "vrm",
    params(
        ("vrm" = String, Path, description = "VRM entity ID"),
        ("bone_name" = String, Path, description = "Bone name"),
    ),
    responses(
        (status = 200, description = "Bone entity ID", body = String),
        (status = 404, description = "VRM or bone not found"),
    ),
)]
pub async fn get(
    State(api): State<VrmApi>,
    Path((vrm, bone_name)): Path<(Entity, String)>,
) -> HttpResult<Entity> {
    let bone_name = VrmBone(bone_name);
    api.bone(vrm, bone_name).await.into_http_result()
}
