use crate::error::{ApiError, ApiResult};
use crate::persona::PersonaApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Persona, PersonaId, PersonaIndex};

impl PersonaApi {
    /// Retrieves a single persona by its ID.
    pub async fn get(&self, persona_id: PersonaId) -> ApiResult<Persona> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get).with(persona_id)).await
            })
            .await?
    }

    /// Lists all persona entities.
    pub async fn list(&self) -> ApiResult<Vec<Persona>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list)).await })
            .await
    }
}

fn get(
    In(persona_id): In<PersonaId>,
    index: Res<PersonaIndex>,
    personas: Query<&Persona>,
) -> ApiResult<Persona> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;
    personas
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::EntityNotFound)
}

fn list(personas: Query<&Persona>) -> Vec<Persona> {
    personas.iter().cloned().collect()
}
