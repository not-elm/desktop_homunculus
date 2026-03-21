use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{CharacterId, CharacterName, CharacterRegistry};
use homunculus_prefs::character_repo::CharacterRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl CharacterApi {
    /// Returns the display name of the character.
    pub async fn get_name(&self, id: CharacterId) -> ApiResult<String> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_character_name).with(id))
                    .await
            })
            .await?
    }

    /// Updates the display name of the character.
    ///
    /// Persists the new name to the database and updates both the
    /// `CharacterName` and Bevy `Name` components.
    pub async fn set_name(&self, id: CharacterId, name: String) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_character_name).with((id, name)))
                    .await
            })
            .await?
    }
}

fn get_character_name(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    names: Query<&CharacterName>,
) -> ApiResult<String> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    let character_name = names
        .get(entity)
        .map_err(|_| ApiError::CharacterNotFound(id.to_string()))?;
    Ok(character_name.0.clone())
}

fn set_character_name(
    In((id, name)): In<(CharacterId, String)>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;

    commands
        .entity(entity)
        .try_insert((CharacterName(name.clone()), Name::new(name.clone())));

    CharacterRepo::new(&db)
        .update_name(&id, &name)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
