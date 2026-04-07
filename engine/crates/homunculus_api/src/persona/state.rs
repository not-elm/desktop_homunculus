use crate::error::{ApiError, ApiResult};
use crate::persona::PersonaApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{PersonaId, PersonaIndex, PersonaState};

impl PersonaApi {
    /// Retrieves the current state of a persona.
    pub async fn state(&self, persona_id: PersonaId) -> ApiResult<PersonaState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_state).with(persona_id))
                    .await
            })
            .await?
    }

    /// Sets the state of a persona.
    pub async fn set_state(&self, persona_id: PersonaId, state: PersonaState) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_state).with((persona_id, state)))
                    .await
            })
            .await?
    }
}

fn get_state(
    In(persona_id): In<PersonaId>,
    index: Res<PersonaIndex>,
    states: Query<&PersonaState>,
) -> ApiResult<PersonaState> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;
    states
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::EntityNotFound)
}

fn set_state(
    In((persona_id, state)): In<(PersonaId, PersonaState)>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
) -> ApiResult {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;
    commands.entity(entity).try_insert(state);
    Ok(())
}
