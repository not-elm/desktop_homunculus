use crate::error::{ApiError, ApiResult};
use crate::persona::vrm_detach::detach_core;
use crate::persona::{PersonaApi, PersonaSnapshot};
use crate::prelude::initialized;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, VrmHandle};
use homunculus_core::prelude::{
    AssetResolver, Persona, PersonaChangeEvent, PersonaId, PersonaIndex, PersonaState,
    VrmAttachedEvent, VrmDetachedEvent, VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Attaches a VRM model to an existing persona entity.
    ///
    /// If a VRM is already attached, it is detached first. The method waits
    /// for deferred despawn commands to flush between detach and attach phases
    /// to prevent stale VRMA entity reuse.
    pub async fn attach_vrm(
        &self,
        persona_id: PersonaId,
        asset_id: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                let (snapshot, entity) = task
                    .will(
                        Update,
                        once::run(detach_phase).with((persona_id, asset_id.clone())),
                    )
                    .await
                    .ok()?;

                task.will(
                    Update,
                    once::run(attach_phase).with((entity, asset_id.clone())),
                )
                .await;

                task.will(Update, wait::until(initialized).with(entity))
                    .await;

                // Broadcast VrmAttachedEvent after everything is ready
                task.will(
                    Update,
                    once::run(broadcast_attached).with((entity, asset_id)),
                )
                .await;

                Some(snapshot)
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }
}

/// Phase 1: Detach old VRM and despawn stale VRMA children.
///
/// Uses [`detach_core`] for the detach step and persists only once — after the
/// new `asset_id` has been set — so SSE consumers never see a transient
/// `vrm_asset_id = None` state.
///
/// Returns the persona snapshot and entity for the attach phase.
fn detach_phase(
    In((persona_id, asset_id)): In<(PersonaId, String)>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
    mut personas: Query<(&mut Persona, &PersonaState)>,
    vrm_handles: Query<&VrmHandle>,
    prefs: NonSend<PrefsDatabase>,
    tx_detached: Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) -> ApiResult<(PersonaSnapshot, Entity)> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    let (mut persona, state) = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    // Detach old VRM if present (no intermediate persist/broadcast)
    let old_asset_id = if vrm_handles.get(entity).is_ok() {
        detach_core(&mut commands, &mut persona, entity)
    } else {
        None
    };

    // Set new asset_id and persist once
    persona.vrm_asset_id = Some(asset_id);
    let updated = persona.clone();
    let state_str = state.0.clone();

    persist_and_broadcast_change(&prefs, &tx_change, entity, &updated);

    // Broadcast detach event if old VRM was removed
    if let Some(old_id) = old_asset_id
        && let Some(tx) = &tx_detached
    {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmDetachedEvent { asset_id: old_id },
        });
    }

    Ok((
        PersonaSnapshot {
            persona: updated,
            state: state_str,
            spawned: true,
        },
        entity,
    ))
}

/// Phase 2: Insert the new VRM handle (runs after deferred despawn has flushed).
fn attach_phase(
    In((entity, asset_id)): In<(Entity, String)>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
    cameras: Cameras,
) {
    let Ok(handle) = asset_resolver.load(&asset_id) else {
        error!("Failed to load VRM asset: {asset_id}");
        return;
    };
    commands.entity(entity).try_insert((
        VrmHandle(handle),
        LookAt::Cursor,
        BodyTracking::default(),
        cameras.all_layers(),
    ));
}

/// Persists persona to DB and broadcasts a change event.
fn persist_and_broadcast_change(
    prefs: &PrefsDatabase,
    tx_change: &Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    entity: Entity,
    persona: &Persona,
) {
    if let Err(e) = prefs.update_persona(persona) {
        warn!("Failed to persist persona after VRM attach: {e}");
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

/// Broadcasts [`VrmAttachedEvent`] after the VRM entity is fully initialized.
fn broadcast_attached(
    In((entity, asset_id)): In<(Entity, String)>,
    tx: Option<Res<VrmEventSender<VrmAttachedEvent>>>,
) {
    if let Some(tx) = tx {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmAttachedEvent { asset_id },
        });
    }
}
