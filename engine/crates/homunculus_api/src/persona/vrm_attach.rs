use crate::error::{ApiError, ApiResult};
use crate::persona::PersonaApi;
use crate::prelude::initialized;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, VrmHandle};
use homunculus_core::prelude::{
    AssetResolver, Persona, PersonaChangeEvent, PersonaId, PersonaIndex, VrmAttachedEvent,
    VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Attaches a VRM model to an existing persona entity.
    pub async fn attach_vrm(&self, persona_id: PersonaId, asset_id: String) -> ApiResult<Persona> {
        self.0
            .schedule(move |task| async move {
                let (persona, entity) = task
                    .will(Update, once::run(attach).with((persona_id, asset_id)))
                    .await
                    .ok()?;
                task.will(Update, wait::until(initialized).with(entity))
                    .await;
                Some(persona)
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
    mut personas: Query<&mut Persona>,
    vrm_handles: Query<&VrmHandle>,
    asset_resolver: AssetResolver,
    cameras: Cameras,
    prefs: NonSend<PrefsDatabase>,
    tx_attached: Option<Res<VrmEventSender<VrmAttachedEvent>>>,
    tx_change: Option<Res<VrmEventSender<PersonaChangeEvent>>>,
) -> ApiResult<(Persona, Entity)> {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    if vrm_handles.get(entity).is_ok() {
        return Err(ApiError::Conflict(
            "VRM already attached to this persona".to_string(),
        ));
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

    let mut persona = personas
        .get_mut(entity)
        .map_err(|_| ApiError::EntityNotFound)?;
    persona.vrm_asset_id = Some(asset_id.clone());
    let updated = persona.clone();

    persist_and_broadcast(
        &prefs,
        &tx_attached,
        &tx_change,
        entity,
        &updated,
        &asset_id,
    );

    Ok((updated, entity))
}

/// Persists persona to DB and broadcasts attach/change events.
fn persist_and_broadcast(
    prefs: &PrefsDatabase,
    tx_attached: &Option<Res<VrmEventSender<VrmAttachedEvent>>>,
    tx_change: &Option<Res<VrmEventSender<PersonaChangeEvent>>>,
    entity: Entity,
    persona: &Persona,
    asset_id: &str,
) {
    if let Err(e) = prefs.save_persona(persona) {
        warn!("Failed to persist persona after VRM attach: {e}");
    }
    if let Some(tx) = tx_attached {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmAttachedEvent {
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
