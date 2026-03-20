use crate::avatar::AvatarApi;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{AvatarId, AvatarName, AvatarState};
use serde::{Deserialize, Serialize};

/// Summary information about a single avatar.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarInfo {
    /// The unique avatar identifier.
    pub id: String,
    /// The display name.
    pub name: String,
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
}

fn list_avatars(
    avatars: Query<(&AvatarId, &AvatarName, &AvatarState, Has<Vrm>)>,
) -> ApiResult<Vec<AvatarInfo>> {
    let infos = avatars
        .iter()
        .map(|(id, name, state, has_vrm)| AvatarInfo {
            id: id.to_string(),
            name: name.0.clone(),
            state: state.0.clone(),
            has_vrm,
        })
        .collect();
    Ok(infos)
}
