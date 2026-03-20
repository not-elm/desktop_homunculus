use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{AvatarId, AvatarName, AvatarRegistry};
use homunculus_prefs::avatar_repo::AvatarRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl AvatarApi {
    /// Returns the display name of the avatar.
    pub async fn get_name(&self, id: AvatarId) -> ApiResult<String> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_avatar_name).with(id)).await
            })
            .await?
    }

    /// Updates the display name of the avatar.
    ///
    /// Persists the new name to the database and updates both the
    /// `AvatarName` and Bevy `Name` components.
    pub async fn set_name(&self, id: AvatarId, name: String) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_avatar_name).with((id, name)))
                    .await
            })
            .await?
    }
}

fn get_avatar_name(
    In(id): In<AvatarId>,
    registry: Res<AvatarRegistry>,
    names: Query<&AvatarName>,
) -> ApiResult<String> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;
    let avatar_name = names
        .get(entity)
        .map_err(|_| ApiError::AvatarNotFound(id.to_string()))?;
    Ok(avatar_name.0.clone())
}

fn set_avatar_name(
    In((id, name)): In<(AvatarId, String)>,
    mut commands: Commands,
    registry: Res<AvatarRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;

    commands
        .entity(entity)
        .try_insert((AvatarName(name.clone()), Name::new(name.clone())));

    AvatarRepo::new(&db)
        .update_name(&id, &name)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
