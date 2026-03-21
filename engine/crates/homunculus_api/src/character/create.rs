use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    AssetId, AssetIdComponent, Character, CharacterId, CharacterName, CharacterRegistry,
    CharacterState, Persona,
};
use homunculus_prefs::character_repo::CharacterRepo;
use homunculus_prefs::prelude::PrefsDatabase;

/// Arguments for creating a new character.
#[derive(Debug, Clone)]
pub(crate) struct CreateCharacterArgs {
    pub id: CharacterId,
    pub asset_id: AssetId,
    pub name: String,
    pub ensure: bool,
}

impl CharacterApi {
    /// Creates a new character entity and persists it to the database.
    ///
    /// When `ensure` is true and a character with the given ID already exists,
    /// the existing entity is returned instead of raising an error.
    pub async fn create(
        &self,
        id: CharacterId,
        asset_id: AssetId,
        name: String,
        ensure: bool,
    ) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                let args = CreateCharacterArgs {
                    id,
                    asset_id,
                    name,
                    ensure,
                };
                task.will(Update, once::run(create_character).with(args))
                    .await
            })
            .await?
    }
}

fn create_character(
    In(args): In<CreateCharacterArgs>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult<Entity> {
    if let Some(entity) = registry.get(&args.id) {
        if args.ensure {
            return Ok(entity);
        }
        return Err(ApiError::CharacterAlreadyExists(args.id.to_string()));
    }

    persist_character(&db, &args)?;

    let entity = commands
        .spawn((
            Character,
            args.id,
            CharacterName(args.name),
            Name::new(String::new()),
            AssetIdComponent(args.asset_id),
            CharacterState::default(),
            Persona::default(),
        ))
        .id();
    Ok(entity)
}

/// Inserts the character row into the database.
fn persist_character(db: &PrefsDatabase, args: &CreateCharacterArgs) -> ApiResult<()> {
    let persona_json =
        serde_json::to_string(&Persona::default()).unwrap_or_else(|_| "{}".to_string());
    CharacterRepo::new(db)
        .create(
            &args.id,
            args.asset_id.as_ref(),
            &args.name,
            &persona_json,
            "{}",
        )
        .map_err(|e| ApiError::Sql(e.to_string()))
}
