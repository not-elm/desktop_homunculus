use axum::extract::{Path, State};
use bevy::prelude::Entity;
use bevy_vrm1::vrm::VrmBone;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::VrmApi;

use crate::route::persona::SpawnedPersonaPath;

/// Get the entity ID of a specific bone in a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/bone/{bone_name}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("bone_name" = String, Path, description = "Bone name"),
    ),
    responses(
        (status = 200, description = "Bone entity ID", body = String),
        (status = 404, description = "Persona, VRM, or bone not found"),
    ),
)]
pub async fn get_bone(
    State(api): State<VrmApi>,
    path: SpawnedPersonaPath,
    Path((_, bone_name)): Path<(String, String)>,
) -> HttpResult<Entity> {
    let bone_name = VrmBone(bone_name);
    api.bone(path.entity, bone_name).await.into_http_result()
}
