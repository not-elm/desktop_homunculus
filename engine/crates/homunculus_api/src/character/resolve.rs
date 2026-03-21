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

    /// Resolves a character ID to its ECS entity and verifies a VRM is attached.
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
}

fn resolve_entity(In(id): In<CharacterId>, registry: Res<CharacterRegistry>) -> ApiResult<Entity> {
    registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))
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
