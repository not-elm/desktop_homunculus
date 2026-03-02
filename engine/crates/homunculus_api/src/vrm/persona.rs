use crate::error::{ApiError, ApiResult};
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    AssetIdComponent, Persona, PersonaChangeEvent, VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::{PrefsDatabase, PrefsKeys};

impl VrmApi {
    /// Retrieves the persona of a VRM entity.
    pub async fn persona(&self, entity: Entity) -> ApiResult<Persona> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_persona).with(entity)).await
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }

    /// Sets the persona of a VRM entity.
    /// Persists to preferences and updates all entities sharing the same asset ID.
    pub async fn set_persona(&self, entity: Entity, persona: Persona) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_persona).with((entity, persona)))
                    .await;
            })
            .await
    }
}

fn get_persona(In(entity): In<Entity>, personas: Query<&Persona>) -> Option<Persona> {
    personas.get(entity).ok().cloned()
}

fn set_persona(
    In((entity, persona)): In<(Entity, Persona)>,
    mut commands: Commands,
    asset_ids: Query<&AssetIdComponent>,
    all_vrms: Query<(Entity, &AssetIdComponent), With<Persona>>,
    prefs: NonSend<PrefsDatabase>,
    tx: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) {
    let Ok(asset_id) = asset_ids.get(entity) else {
        return;
    };
    let key = PrefsKeys::persona(asset_id.0.as_ref());
    let _ = prefs.save_as(&key, &persona);

    for (e, aid) in all_vrms.iter() {
        if aid.0 == asset_id.0 {
            commands.entity(e).try_insert(persona.clone());
            if let Some(tx) = &tx {
                let _ = tx.try_broadcast(VrmEvent {
                    vrm: e,
                    payload: PersonaChangeEvent {
                        persona: persona.clone(),
                    },
                });
            }
        }
    }
}
