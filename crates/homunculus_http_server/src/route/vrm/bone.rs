use axum::extract::{Path, State};
use bevy::prelude::Entity;
use bevy_vrm1::vrm::VrmBone;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::VrmApi;

/// Fetch all VRM models.
pub async fn get(
    State(api): State<VrmApi>,
    Path((vrm, bone_name)): Path<(u64, String)>,
) -> HttpResult<u64> {
    let vrm = Entity::from_bits(vrm);
    let bone_name = VrmBone(bone_name);
    api.bone(vrm, bone_name)
        .await
        .map(|e| e.to_bits())
        .into_http_result()
}
