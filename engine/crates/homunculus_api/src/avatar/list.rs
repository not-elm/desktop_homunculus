use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{
    AssetIdComponent, AvatarId, AvatarName, AvatarRegistry, AvatarState,
};
use serde::{Deserialize, Serialize};

/// Summary information about a single avatar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct AvatarInfo {
    /// The unique avatar identifier.
    pub id: String,
    /// The display name.
    pub name: String,
    /// The asset ID of the VRM model bound to this avatar.
    pub asset_id: String,
    /// The current behavioral state (e.g. "idle", "sitting").
    pub state: String,
    /// Whether a VRM model is currently attached.
    pub has_vrm: bool,
}

impl AvatarApi {
    /// Lists all registered avatars with summary information.
    pub async fn list(&self) -> ApiResult<Vec<AvatarInfo>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_avatars)).await })
            .await?
    }

    /// Returns summary information for a single avatar.
    pub async fn get_info(&self, id: AvatarId) -> ApiResult<AvatarInfo> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_avatar_info).with(id)).await
            })
            .await?
    }
}

fn list_avatars(
    avatars: Query<(
        &AvatarId,
        &AvatarName,
        &AssetIdComponent,
        &AvatarState,
        Has<Vrm>,
    )>,
) -> ApiResult<Vec<AvatarInfo>> {
    let infos = avatars
        .iter()
        .map(|(id, name, asset_id, state, has_vrm)| {
            to_avatar_info(id, name, asset_id, state, has_vrm)
        })
        .collect();
    Ok(infos)
}

fn get_avatar_info(
    In(id): In<AvatarId>,
    registry: Res<AvatarRegistry>,
    avatars: Query<(
        &AvatarId,
        &AvatarName,
        &AssetIdComponent,
        &AvatarState,
        Has<Vrm>,
    )>,
) -> ApiResult<AvatarInfo> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;
    let (id, name, asset_id, state, has_vrm) = avatars
        .get(entity)
        .map_err(|_| ApiError::AvatarNotFound(id.to_string()))?;
    Ok(to_avatar_info(id, name, asset_id, state, has_vrm))
}

fn to_avatar_info(
    id: &AvatarId,
    name: &AvatarName,
    asset_id: &AssetIdComponent,
    state: &AvatarState,
    has_vrm: bool,
) -> AvatarInfo {
    AvatarInfo {
        id: id.to_string(),
        name: name.0.clone(),
        asset_id: asset_id.0.as_ref().to_string(),
        state: state.0.clone(),
        has_vrm,
    }
}
