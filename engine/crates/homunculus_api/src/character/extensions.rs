use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{CharacterId, CharacterRegistry};
use homunculus_prefs::character_repo::CharacterRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl CharacterApi {
    /// Returns extension data for a specific mod on the character.
    ///
    /// Returns the JSON data previously stored by the mod, or a
    /// `NotFoundPreferences` error if no data exists.
    pub async fn get_extension(
        &self,
        id: CharacterId,
        mod_name: String,
    ) -> ApiResult<serde_json::Value> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_ext).with((id, mod_name)))
                    .await
            })
            .await?
    }

    /// Stores extension data for a specific mod on the character.
    ///
    /// Creates or replaces any existing data for the given mod.
    pub async fn set_extension(
        &self,
        id: CharacterId,
        mod_name: String,
        data: serde_json::Value,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_ext).with((id, mod_name, data)))
                    .await
            })
            .await?
    }

    /// Deletes extension data for a specific mod on the character.
    pub async fn delete_extension(&self, id: CharacterId, mod_name: String) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(delete_ext).with((id, mod_name)))
                    .await
            })
            .await?
    }
}

fn get_ext(
    In((id, mod_name)): In<(CharacterId, String)>,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult<serde_json::Value> {
    require_character(&registry, &id)?;

    let data_json = CharacterRepo::new(&db)
        .get_extension(&id, &mod_name)
        .map_err(|e| ApiError::Sql(e.to_string()))?
        .ok_or_else(|| ApiError::NotFoundPreferences(format!("extension:{id}:{mod_name}")))?;

    serde_json::from_str(&data_json).map_err(|e| ApiError::FailedLoad(e.to_string()))
}

fn set_ext(
    In((id, mod_name, data)): In<(CharacterId, String, serde_json::Value)>,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    require_character(&registry, &id)?;

    let data_json =
        serde_json::to_string(&data).map_err(|e| ApiError::FailedSave(e.to_string()))?;

    CharacterRepo::new(&db)
        .set_extension(&id, &mod_name, &data_json)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}

fn delete_ext(
    In((id, mod_name)): In<(CharacterId, String)>,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    require_character(&registry, &id)?;

    CharacterRepo::new(&db)
        .delete_extension(&id, &mod_name)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}

/// Validates that the character exists in the registry.
fn require_character(registry: &CharacterRegistry, id: &CharacterId) -> ApiResult<()> {
    registry
        .get(id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    Ok(())
}
