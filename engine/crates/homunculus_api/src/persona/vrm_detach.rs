use crate::error::{ApiError, ApiResult};
use crate::persona::PersonaApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::RequestDetachVrm;
use homunculus_core::prelude::{
    Persona, PersonaChangeEvent, PersonaId, PersonaIndex, VrmDetachedEvent, VrmEvent,
    VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

/// Clears VRM from persona and triggers [`RequestDetachVrm`].
///
/// This is a mutation-only helper shared by both the standalone `detach_vrm`
/// API and the auto-detach path inside `attach_vrm`. Persistence and event
/// broadcasting are left to the caller so each call-site can choose its own
/// policy (e.g. attach skips the intermediate persist).
///
/// Returns the old `asset_id` if a VRM was attached, or `None` otherwise.
pub(super) fn detach_core(
    commands: &mut Commands,
    persona: &mut Persona,
    entity: Entity,
) -> Option<String> {
    let old_asset_id = persona.vrm_asset_id.take()?;
    commands.entity(entity).trigger(RequestDetachVrm);
    Some(old_asset_id)
}

impl PersonaApi {
    /// Detaches the VRM model from a persona, keeping the persona entity intact.
    pub async fn detach_vrm(&self, persona_id: PersonaId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(detach).with(persona_id)).await
            })
            .await?
    }
}

fn detach(
    In(persona_id): In<PersonaId>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
    mut personas: Query<&mut Persona>,
    prefs: NonSend<PrefsDatabase>,
    tx_detached: Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) -> ApiResult {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    let mut persona = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let asset_id = detach_core(&mut commands, &mut persona, entity)
        .ok_or_else(|| ApiError::Conflict("No VRM attached to this persona".to_string()))?;

    let updated = persona.clone();
    persist_and_broadcast(
        &prefs,
        &tx_detached,
        &tx_change,
        entity,
        &updated,
        &asset_id,
    );

    Ok(())
}

/// Persists persona to DB and broadcasts detach/change events.
fn persist_and_broadcast(
    prefs: &PrefsDatabase,
    tx_detached: &Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: &Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    entity: Entity,
    persona: &Persona,
    asset_id: &str,
) {
    if let Err(e) = prefs.update_persona(persona) {
        warn!("Failed to persist persona after VRM detach: {e}");
    }
    if let Some(tx) = tx_detached {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmDetachedEvent {
                asset_id: asset_id.to_string(),
            },
        });
    }
    if let Some(tx) = tx_change {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaChangeEvent {
                persona: persona.clone(),
            },
        });
    }
}
