use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{AvatarId, AvatarRegistry, AvatarState};
use homunculus_prefs::avatar_repo::AvatarRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl AvatarApi {
    /// Returns the current behavioral state of the avatar.
    pub async fn get_state(&self, id: AvatarId) -> ApiResult<AvatarState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_avatar_state).with(id))
                    .await
            })
            .await?
    }

    /// Updates the behavioral state of the avatar.
    ///
    /// Persists the new state to the database and updates the `AvatarState`
    /// component on the entity.
    pub async fn set_state(&self, id: AvatarId, state: AvatarState) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_avatar_state).with((id, state)))
                    .await
            })
            .await?
    }
}

fn get_avatar_state(
    In(id): In<AvatarId>,
    registry: Res<AvatarRegistry>,
    states: Query<&AvatarState>,
) -> ApiResult<AvatarState> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;
    states
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::AvatarNotFound(id.to_string()))
}

fn set_avatar_state(
    In((id, state)): In<(AvatarId, AvatarState)>,
    mut commands: Commands,
    registry: Res<AvatarRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;

    commands.entity(entity).try_insert(state.clone());

    AvatarRepo::new(&db)
        .update_state(&id, &state)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
