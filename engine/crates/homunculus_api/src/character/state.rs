use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{CharacterId, CharacterRegistry, CharacterState};
use homunculus_prefs::character_repo::CharacterRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl CharacterApi {
    /// Returns the current behavioral state of the character.
    pub async fn get_state(&self, id: CharacterId) -> ApiResult<CharacterState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_character_state).with(id))
                    .await
            })
            .await?
    }

    /// Updates the behavioral state of the character.
    ///
    /// Persists the new state to the database and updates the `CharacterState`
    /// component on the entity.
    pub async fn set_state(&self, id: CharacterId, state: CharacterState) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_character_state).with((id, state)))
                    .await
            })
            .await?
    }
}

fn get_character_state(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    states: Query<&CharacterState>,
) -> ApiResult<CharacterState> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    states
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::CharacterNotFound(id.to_string()))
}

fn set_character_state(
    In((id, state)): In<(CharacterId, CharacterState)>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;

    commands.entity(entity).try_insert(state.clone());

    CharacterRepo::new(&db)
        .update_state(&id, &state)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
