use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use crate::prelude::initialized;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, RequestDetachVrm, VrmHandle};
use homunculus_core::prelude::{
    AssetResolver, Persona, PersonaChangeEvent, PersonaId, PersonaIndex, PersonaState,
    VrmAttachedEvent, VrmDetachedEvent, VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Attaches a VRM model to an existing persona entity.
    pub async fn attach_vrm(
        &self,
        persona_id: PersonaId,
        asset_id: String,
    ) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                let (snapshot, entity, asset_id) = task
                    .will(Update, once::run(attach).with((persona_id, asset_id)))
                    .await
                    .ok()?;
                task.will(Update, wait::until(initialized).with(entity))
                    .await;
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

#[allow(clippy::too_many_arguments)]
fn attach(
    In((persona_id, asset_id)): In<(PersonaId, String)>,
    mut commands: Commands,
    index: Res<PersonaIndex>,
    mut personas: Query<(&mut Persona, &PersonaState)>,
    vrm_handles: Query<&VrmHandle>,
    asset_resolver: AssetResolver,
    cameras: Cameras,
    prefs: NonSend<PrefsDatabase>,
    tx_detached: Option<Res<VrmEventSender<VrmDetachedEvent>>>,
    tx_change: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) -> ApiResult<(PersonaSnapshot, Entity, String)> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    if vrm_handles.get(entity).is_ok() {
        auto_detach(
            &mut commands,
            &mut personas,
            &prefs,
            &tx_detached,
            &tx_change,
            entity,
        )?;
    }

    let handle = asset_resolver
        .load(&asset_id)
        .map_err(|_| ApiError::AssetNotFound(asset_id.clone().into()))?;

    commands.entity(entity).try_insert((
        VrmHandle(handle),
        LookAt::Cursor,
        BodyTracking::default(),
        cameras.all_layers(),
    ));

    let (mut persona, state) = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;
    persona.vrm_asset_id = Some(asset_id.clone());
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
        asset_id,
    ))
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
