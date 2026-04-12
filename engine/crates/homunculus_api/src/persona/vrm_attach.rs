use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use crate::prelude::initialized;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, RequestDetachVrm, VrmHandle};
use homunculus_core::prelude::{
    AssetIdComponent, AssetResolver, Persona, PersonaChangeEvent, PersonaId, PersonaIndex,
    PersonaState, VrmAttachedEvent, VrmDetachedEvent, VrmEvent, VrmEventSender,
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
                // Phase 1: detach old VRM + despawn old VRMA children (all deferred)
                let (snapshot, entity) = task
                    .will(
                        Update,
                        once::run(detach_phase).with((persona_id, asset_id.clone())),
                    )
                    .await
                    .ok()?;

                // Flush deferred commands (despawn old children, trigger RequestDetachVrm)
                task.will(Update, once::run(|| {})).await;

                // Phase 2: insert new VRM handle (now that old children are gone)
                task.will(
                    Update,
                    once::run(attach_phase).with((entity, asset_id.clone())),
                )
                .await;

                // Wait for VRM initialization
                task.will(Update, wait::until(initialized).with(entity))
                    .await;

                // Wait for animation graph construction
                // (RequestUpdateAnimationGraph is a deferred trigger from trigger_loaded)
                task.will(Update, once::run(|| {})).await;

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
/// Returns the persona snapshot and entity for the attach phase.
#[allow(clippy::too_many_arguments)]
fn detach_phase(
    In((persona_id, asset_id)): In<(PersonaId, String)>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
    mut personas: Query<(&mut Persona, &PersonaState)>,
    vrm_handles: Query<&VrmHandle>,
    prefs: NonSend<PrefsDatabase>,
    tx_detached: Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    children_query: Query<&Children>,
    vrma_entities: Query<Entity, With<AssetIdComponent>>,
) -> ApiResult<(PersonaSnapshot, Entity)> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    if vrm_handles.get(entity).is_ok() {
        despawn_vrma_children(&mut commands, &children_query, &vrma_entities, entity);
        auto_detach(
            &mut commands,
            &mut personas,
            &prefs,
            &tx_detached,
            &tx_change,
            entity,
        )?;
    }

    // Update persona DB record with new VRM asset ID
    let (mut persona, state) = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;
    persona.vrm_asset_id = Some(asset_id);
    let updated = persona.clone();
    let state_str = state.0.clone();

    persist_and_broadcast_change(&prefs, &tx_change, entity, &updated);

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

/// Despawns VRMA child entities before VRM detach.
fn despawn_vrma_children(
    commands: &mut Commands,
    children_query: &Query<&Children>,
    vrma_entities: &Query<Entity, With<AssetIdComponent>>,
    entity: Entity,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            if vrma_entities.get(child).is_ok() {
                commands.entity(child).despawn();
            }
        }
    }
}

/// Detaches the currently attached VRM before a new one is attached.
fn auto_detach(
    commands: &mut Commands,
    personas: &mut Query<(&mut Persona, &PersonaState)>,
    prefs: &PrefsDatabase,
    tx_detached: &Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: &Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    entity: Entity,
) -> ApiResult<()> {
    let (mut persona, _) = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;

    let old_asset_id = persona.vrm_asset_id.take().unwrap_or_default();
    let updated = persona.clone();

    commands.entity(entity).trigger(RequestDetachVrm);

    if let Err(e) = prefs.update_persona(&updated) {
        warn!("Failed to persist persona after VRM auto-detach: {e}");
    }
    if let Some(tx) = tx_detached {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmDetachedEvent {
                asset_id: old_asset_id,
            },
        });
    }
    if let Some(tx) = tx_change {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaChangeEvent { persona: updated },
        });
    }

    Ok(())
}
