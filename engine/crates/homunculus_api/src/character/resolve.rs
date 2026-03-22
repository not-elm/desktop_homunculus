use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{CharacterId, CharacterRegistry};

impl CharacterApi {
    /// Resolves a character ID to its ECS entity.
    ///
    /// Returns `CharacterNotFound` if no entity is registered for the given ID.
    pub async fn resolve(&self, id: CharacterId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(resolve_entity).with(id)).await
            })
            .await?
    }

    ///
    /// Returns `CharacterNotFound` if no entity is registered, or `VrmNotAttached`
    /// if the entity does not have the `Vrm` marker component.
    pub async fn resolve_with_vrm(&self, id: CharacterId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(resolve_entity_with_vrm).with(id))
                    .await
            })
            .await?
    }

    /// Checks whether a VRM model is attached to the given character.
    pub async fn has_vrm(&self, id: CharacterId) -> ApiResult<bool> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(check_has_vrm).with(id)).await
            })
            .await?
    }

    /// Looks up the character ID for the given entity in a single ECS round-trip.
    ///
    /// Returns `None` if the entity is not registered in the [`CharacterRegistry`].
    pub async fn id_for_entity(&self, entity: Entity) -> Option<CharacterId> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(find_id_for_entity).with(entity))
                    .await
            })
            .await
            .ok()
            .flatten()
    }

    /// Returns the first registered character as `(CharacterId, Entity)` in a single ECS round-trip.
    ///
    /// Returns `None` if no characters are currently registered.
    pub async fn first_character(&self) -> Option<(CharacterId, Entity)> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(query_first_character)).await
            })
            .await
            .ok()
            .flatten()
    }
}

fn resolve_entity(In(id): In<CharacterId>, registry: Res<CharacterRegistry>) -> ApiResult<Entity> {
    registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))
}

fn find_id_for_entity(
    In(entity): In<Entity>,
    registry: Res<CharacterRegistry>,
) -> Option<CharacterId> {
    registry.get_id(entity).cloned()
}

fn query_first_character(registry: Res<CharacterRegistry>) -> Option<(CharacterId, Entity)> {
    registry
        .iter()
        .next()
        .map(|(id, &entity)| (id.clone(), entity))
}

fn resolve_entity_with_vrm(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    vrm_check: Query<(), With<Vrm>>,
) -> ApiResult<Entity> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    if vrm_check.get(entity).is_err() {
        return Err(ApiError::VrmNotAttached(id.to_string()));
    }
    Ok(entity)
}

fn check_has_vrm(
    In(id): In<CharacterId>,
    registry: Res<CharacterRegistry>,
    vrm_check: Query<(), With<Vrm>>,
) -> ApiResult<bool> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;
    Ok(vrm_check.get(entity).is_ok())
}
