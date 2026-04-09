use crate::error::{ApiError, ApiResult};
use crate::persona::PersonaApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    PersonaDeletedEvent, PersonaId, PersonaIndex, VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Deletes a persona entity and removes it from the database.
    pub async fn delete(&self, persona_id: PersonaId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(delete).with(persona_id)).await
            })
            .await?
    }
}

fn delete(
    In(persona_id): In<PersonaId>,
    mut commands: Commands,
    mut index: ResMut<PersonaIndex>,
    prefs: NonSend<PrefsDatabase>,
    tx: Option<Res<VrmEventSender<PersonaDeletedEvent>>>,
) -> ApiResult {
    delete_from_db(&prefs, &persona_id)?;

    let entity = despawn_if_spawned(&mut commands, &mut index, &persona_id);
    broadcast_deleted(&tx, entity, &persona_id);

    Ok(())
}

/// Deletes the persona from the database, returning 404 if it did not exist.
fn delete_from_db(prefs: &PrefsDatabase, persona_id: &PersonaId) -> ApiResult {
    let affected = prefs
        .delete_persona(persona_id.as_ref())
        .map_err(|e| ApiError::Sql(e.to_string()))?;
    if affected == 0 {
        return Err(ApiError::EntityNotFound);
    }
    Ok(())
}

/// Despawns the ECS entity if it exists, returning the entity or a placeholder.
fn despawn_if_spawned(
    commands: &mut Commands,
    index: &mut ResMut<PersonaIndex>,
    persona_id: &PersonaId,
) -> Entity {
    if let Some(entity) = index.get(persona_id) {
        commands.entity(entity).try_despawn();
        index.remove(persona_id);
        entity
    } else {
        Entity::PLACEHOLDER
    }
}

/// Broadcasts a [`PersonaDeletedEvent`] to all listeners.
fn broadcast_deleted(
    tx: &Option<Res<VrmEventSender<PersonaDeletedEvent>>>,
    entity: Entity,
    persona_id: &PersonaId,
) {
    if let Some(tx) = tx {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaDeletedEvent {
                persona_id: persona_id.clone(),
            },
        });
    }
}
