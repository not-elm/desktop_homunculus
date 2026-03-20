use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{AvatarId, AvatarRegistry};
use homunculus_prefs::avatar_repo::AvatarRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl AvatarApi {
    /// Destroys an avatar entity and removes it from the database.
    pub async fn destroy(&self, id: AvatarId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(destroy_avatar).with(id)).await
            })
            .await?
    }
}

fn destroy_avatar(
    In(id): In<AvatarId>,
    mut commands: Commands,
    registry: Res<AvatarRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;

    commands.entity(entity).try_despawn();

    AvatarRepo::new(&db)
        .delete(&id)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
