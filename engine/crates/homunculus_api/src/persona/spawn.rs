use crate::error::{ApiError, ApiResult};
use crate::persona::{PersonaApi, PersonaSnapshot};
use crate::prelude::initialized;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, VrmHandle};
use homunculus_core::prelude::{
    AssetResolver, Persona, PersonaDespawnedEvent, PersonaId, PersonaIndex, PersonaSpawnedEvent,
    PersonaState, VrmAttachedEvent, VrmEvent, VrmEventSender,
};
use homunculus_prefs::prelude::PrefsDatabase;

impl PersonaApi {
    /// Spawns an ECS entity for a persona that exists in the database.
    ///
    /// If the persona has a `vrm_asset_id`, attaches the VRM model and waits
    /// for initialization so the character appears on screen immediately.
    ///
    /// Returns error if already spawned or persona doesn't exist in DB.
    pub async fn spawn(&self, persona_id: PersonaId) -> ApiResult<PersonaSnapshot> {
        self.0
            .schedule(move |task| async move {
                let (snapshot, entity, vrm_asset_id) = task
                    .will(Update, once::run(spawn_entity).with(persona_id))
                    .await?;

                if let Some(asset_id) = vrm_asset_id {
                    task.will(
                        Update,
                        once::run(attach_vrm_to_entity).with((entity, asset_id)),
                    )
                    .await?;
                    task.will(Update, wait::until(initialized).with(entity))
                        .await;
                }

                Ok(snapshot)
            })
            .await?
    }

    /// Despawns the ECS entity, retaining the DB record.
    ///
    /// Returns error if not spawned.
    pub async fn despawn(&self, persona_id: PersonaId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(despawn).with(persona_id)).await
            })
            .await?
    }
}

fn spawn_entity(
    In(persona_id): In<PersonaId>,
    mut commands: Commands,
    mut index: ResMut<PersonaIndex>,
    prefs: NonSend<PrefsDatabase>,
    tx: Option<Res<VrmEventSender<PersonaSpawnedEvent>>>,
) -> ApiResult<(PersonaSnapshot, Entity, Option<String>)> {
    if index.get(&persona_id).is_some() {
        return Err(ApiError::Conflict(format!(
            "Persona already spawned: {}",
            persona_id
        )));
    }

    let persona = load_persona_from_db(&prefs, &persona_id)?;
    let vrm_asset_id = persona.vrm_asset_id.clone();
    let display_name = persona.name.clone().unwrap_or_else(|| persona_id.0.clone());
    let transform = extract_transform(&persona);
    let state = PersonaState::default();

    let entity = commands
        .spawn((
            persona.clone(),
            state.clone(),
            Name::new(display_name),
            transform,
        ))
        .id();

    index.insert(persona_id.clone(), entity);
    broadcast_spawned(&tx, entity, &persona_id);

    Ok((
        PersonaSnapshot {
            persona,
            state: state.0,
            spawned: true,
        },
        entity,
        vrm_asset_id,
    ))
}

/// Attaches VRM components to an already-spawned entity.
///
/// Unlike the full `attach` in `vrm_attach`, this skips auto-detach (the entity
/// is freshly spawned) and DB persistence (the `vrm_asset_id` is already set).
fn attach_vrm_to_entity(
    In((entity, asset_id)): In<(Entity, String)>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
    cameras: Cameras,
    tx_attached: Option<Res<VrmEventSender<VrmAttachedEvent>>>,
) -> ApiResult<()> {
    let handle = asset_resolver
        .load(&asset_id)
        .map_err(|_| ApiError::AssetNotFound(asset_id.clone().into()))?;

    commands.entity(entity).try_insert((
        VrmHandle(handle),
        LookAt::Cursor,
        BodyTracking::default(),
        cameras.all_layers(),
    ));

    if let Some(tx) = &tx_attached {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: VrmAttachedEvent { asset_id },
        });
    }

    Ok(())
}

fn despawn(
    In(persona_id): In<PersonaId>,
    mut commands: Commands,
    mut index: ResMut<PersonaIndex>,
    tx: Option<Res<VrmEventSender<PersonaDespawnedEvent>>>,
) -> ApiResult {
    let entity = index.get(&persona_id).ok_or(ApiError::EntityNotFound)?;

    commands.entity(entity).try_despawn();
    index.remove(&persona_id);
    broadcast_despawned(&tx, entity, &persona_id);

    Ok(())
}

/// Loads a persona from the database, returning 404 if not found.
fn load_persona_from_db(prefs: &PrefsDatabase, id: &PersonaId) -> ApiResult<Persona> {
    prefs
        .load_persona(&id.0)
        .map_err(|e| ApiError::Sql(e.to_string()))?
        .ok_or(ApiError::EntityNotFound)
}

/// Extracts a [`Transform`] from the persona's metadata `"transform"` key.
///
/// Returns [`Transform::default()`] if the key is absent or cannot be deserialized.
fn extract_transform(persona: &Persona) -> Transform {
    persona
        .metadata
        .get("transform")
        .and_then(|v| serde_json::from_value::<Transform>(v.clone()).ok())
        .unwrap_or_default()
}

/// Broadcasts a [`PersonaSpawnedEvent`] to all listeners.
fn broadcast_spawned(
    tx: &Option<Res<VrmEventSender<PersonaSpawnedEvent>>>,
    entity: Entity,
    persona_id: &PersonaId,
) {
    if let Some(tx) = tx {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaSpawnedEvent {
                persona_id: persona_id.clone(),
            },
        });
    }
}

/// Broadcasts a [`PersonaDespawnedEvent`] to all listeners.
fn broadcast_despawned(
    tx: &Option<Res<VrmEventSender<PersonaDespawnedEvent>>>,
    entity: Entity,
    persona_id: &PersonaId,
) {
    if let Some(tx) = tx {
        let _ = tx.try_broadcast(VrmEvent {
            vrm: entity,
            payload: PersonaDespawnedEvent {
                persona_id: persona_id.clone(),
            },
        });
    }
}
