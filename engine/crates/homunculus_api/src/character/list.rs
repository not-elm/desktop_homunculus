use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{
    AssetIdComponent, CharacterId, CharacterName, CharacterRegistry, CharacterState,
};
use serde::{Deserialize, Serialize};

/// Summary information about a single character.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct CharacterInfo {
    /// The unique character identifier.
    pub id: String,
    /// The display name.
    pub name: String,
    /// The asset ID of the VRM model bound to this character.
    pub asset_id: String,
    /// The current behavioral state (e.g. "idle", "sitting").
    pub state: String,
    /// Whether a VRM model is currently attached.
    pub has_vrm: bool,
}

impl CharacterApi {
    /// Lists all registered characters with summary information.
    pub async fn list(&self) -> ApiResult<Vec<CharacterInfo>> {
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(list_characters)).await },
            )
            .await?
    }

    /// Returns summary information for a single character.
    pub async fn get_info(&self, id: CharacterId) -> ApiResult<CharacterInfo> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_character_info).with(id))
                    .await
            })
            .await?
    }
}

fn list_characters(
    characters: Query<(
        &CharacterId,
        &CharacterName,
        &AssetIdComponent,
        &CharacterState,
        Has<Vrm>,
    )>,
) -> ApiResult<Vec<CharacterInfo>> {
    let infos = characters
        .iter()
        .map(|(id, name, asset_id, state, has_vrm)| {
            to_character_info(id, name, asset_id, state, has_vrm)
        })
        .collect();
    Ok(infos)
}

fn get_character_info(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    characters: Query<(
        &CharacterId,
        &CharacterName,
        &AssetIdComponent,
        &CharacterState,
        Has<Vrm>,
    )>,
) -> ApiResult<CharacterInfo> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    let (id, name, asset_id, state, has_vrm) = characters
        .get(entity)
        .map_err(|_| ApiError::CharacterNotFound(id.to_string()))?;
    Ok(to_character_info(id, name, asset_id, state, has_vrm))
}

fn to_character_info(
    id: &CharacterId,
    name: &CharacterName,
    asset_id: &AssetIdComponent,
    state: &CharacterState,
    has_vrm: bool,
) -> CharacterInfo {
    CharacterInfo {
        id: id.to_string(),
        name: name.0.clone(),
        asset_id: asset_id.0.as_ref().to_string(),
        state: state.0.clone(),
        has_vrm,
    }
}
