use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Persona, PersonaId, PersonaIndex, PersonaState};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Resolves a [`PersonaId`] to its ECS entity.
    pub async fn resolve(&self, persona_id: PersonaId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(resolve).with(persona_id)).await
            })
            .await?
    }

    /// Retrieves a single persona by its ID from the database.
    pub async fn get(&self, persona_id: PersonaId) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get).with(persona_id)).await
            })
            .await?
    }

    /// Lists all personas from the database, overlaying ECS state for spawned ones.
    pub async fn list(&self) -> ApiResult<Vec<PersonaSnapshot>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list)).await })
            .await?
    }
}

fn resolve(In(persona_id): In<PersonaId>, index: Res<PersonaIndex>) -> ApiResult<Entity> {
    index.get(&persona_id).ok_or(ApiError::EntityNotFound)
}

fn get(
    In(persona_id): In<PersonaId>,
    index: Res<PersonaIndex>,
    personas: Query<&PersonaState>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult<PersonaSnapshot> {
    let persona = load_persona_from_db(&prefs, &persona_id)?;
    let (spawned, state) = spawned_state(&index, &personas, &persona_id);

    Ok(PersonaSnapshot {
        persona,
        state,
        spawned,
    })
}

fn list(
    index: Res<PersonaIndex>,
    states: Query<&PersonaState>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult<Vec<PersonaSnapshot>> {
    let db_personas = prefs
        .list_personas()
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(db_personas
        .into_iter()
        .map(|persona| {
            let (spawned, state) = spawned_state(&index, &states, &persona.id);
            PersonaSnapshot {
                persona,
                state,
                spawned,
            }
        })
        .collect())
}

/// Loads a persona from the database, returning 404 if not found.
fn load_persona_from_db(prefs: &PrefsDatabase, id: &PersonaId) -> ApiResult<Persona> {
    prefs
        .load_persona(&id.0)
        .map_err(|e| ApiError::Sql(e.to_string()))?
        .ok_or(ApiError::EntityNotFound)
}

/// Returns `(spawned, state_string)` for a persona by checking the ECS index.
fn spawned_state(
    index: &PersonaIndex,
    states: &Query<&PersonaState>,
    persona_id: &PersonaId,
) -> (bool, String) {
    match index.get(persona_id) {
        Some(entity) => {
            let state = states.get(entity).map(|s| s.0.clone()).unwrap_or_default();
            (true, state)
        }
        None => (false, String::new()),
    }
}
