use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{CharacterId, CharacterRegistry, Persona};
use homunculus_prefs::characters::CharactersTable;
use homunculus_prefs::prelude::PrefsDatabase;

impl CharacterApi {
    /// Returns the persona of the character.
    pub async fn get_persona(&self, id: CharacterId) -> ApiResult<Persona> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_character_persona).with(id))
                    .await
            })
            .await?
    }

    /// Updates the persona of the character.
    ///
    /// Persists the new persona to the database and updates the `Persona`
    /// component on the character entity.
    pub async fn set_persona(&self, id: CharacterId, persona: Persona) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_character_persona).with((id, persona)))
                    .await
            })
            .await?
    }
}

fn get_character_persona(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    personas: Query<&Persona>,
) -> ApiResult<Persona> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    personas
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::CharacterNotFound(id.to_string()))
}

fn set_character_persona(
    In((id, persona)): In<(CharacterId, Persona)>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;

    commands.entity(entity).try_insert(persona.clone());

    let persona_json = serde_json::to_string(&persona).unwrap_or_else(|_| "{}".to_string());
    CharactersTable::new(&db)
        .update_persona(&id, &persona_json)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
