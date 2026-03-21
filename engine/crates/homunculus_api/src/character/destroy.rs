use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{CharacterId, CharacterRegistry};
use homunculus_prefs::character_repo::CharacterRepo;
use homunculus_prefs::prelude::PrefsDatabase;

impl CharacterApi {
    /// Destroys a character entity and removes it from the database.
    pub async fn destroy(&self, id: CharacterId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(destroy_character).with(id))
                    .await
            })
            .await?
    }
}

fn destroy_character(
    In(id): In<CharacterId>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;

    commands.entity(entity).try_despawn();

    CharacterRepo::new(&db)
        .delete(&id)
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(())
}
