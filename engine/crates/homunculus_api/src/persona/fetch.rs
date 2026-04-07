use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Persona, PersonaId, PersonaIndex, PersonaState};

impl PersonaApi {
    /// Resolves a [`PersonaId`] to its ECS entity.
    pub async fn resolve(&self, persona_id: PersonaId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(resolve).with(persona_id)).await
            })
            .await?
    }

    /// Retrieves a single persona by its ID.
    pub async fn get(&self, persona_id: PersonaId) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get).with(persona_id)).await
            })
            .await?
    }

    /// Lists all persona entities.
    pub async fn list(&self) -> ApiResult<Vec<PersonaSnapshot>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list)).await })
            .await
    }
}

fn resolve(In(persona_id): In<PersonaId>, index: Res<PersonaIndex>) -> ApiResult<Entity> {
    index.get(&persona_id).ok_or(ApiError::EntityNotFound)
}

fn get(
    In(persona_id): In<PersonaId>,
    index: Res<PersonaIndex>,
    personas: Query<(&Persona, &PersonaState)>,
) -> ApiResult<PersonaSnapshot> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;
    let (persona, state) = personas
        .get(entity)
        .map_err(|_| ApiError::EntityNotFound)?;
    Ok(PersonaSnapshot {
        persona: persona.clone(),
        state: state.0.clone(),
    })
}

fn list(personas: Query<(&Persona, &PersonaState)>) -> Vec<PersonaSnapshot> {
    personas
        .iter()
        .map(|(persona, state)| PersonaSnapshot {
            persona: persona.clone(),
            state: state.0.clone(),
        })
        .collect()
}
