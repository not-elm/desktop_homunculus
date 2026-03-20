use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{AvatarId, AvatarRegistry, Persona};
use homunculus_prefs::avatar_repo::AvatarRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl AvatarApi {
    /// Returns the persona of the avatar.
    pub async fn get_persona(&self, id: AvatarId) -> ApiResult<Persona> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_avatar_persona).with(id))
                    .await
            })
            .await?
    }

    /// Updates the persona of the avatar.
    ///
    /// Persists the new persona to the database and updates the `Persona`
    /// component on the avatar entity.
    pub async fn set_persona(&self, id: AvatarId, persona: Persona) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_avatar_persona).with((id, persona)))
                    .await
            })
            .await?
    }
}

fn get_avatar_persona(
    In(id): In<AvatarId>,
    registry: Res<AvatarRegistry>,
    personas: Query<&Persona>,
) -> ApiResult<Persona> {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;
    personas
        .get(entity)
        .cloned()
        .map_err(|_| ApiError::AvatarNotFound(id.to_string()))
}

fn set_avatar_persona(
    In((id, persona)): In<(AvatarId, Persona)>,
    mut commands: Commands,
    registry: Res<AvatarRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::AvatarNotFound(id.to_string()))?;

    commands.entity(entity).try_insert(persona.clone());

    let persona_json = serde_json::to_string(&persona).unwrap_or_else(|_| "{}".to_string());
    AvatarRepo::new(&db)
        .update_persona(&id, &persona_json)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
