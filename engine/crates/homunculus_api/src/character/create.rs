use crate::character::CharacterApi;
use crate::character::list::CharacterInfo;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    AssetIdComponent, Character, CharacterId, CharacterName, CharacterRegistry, CharacterState,
    Persona,
};
use homunculus_prefs::characters::CharactersTable;
use homunculus_prefs::prelude::PrefsDatabase;

/// Arguments for creating a new character.
#[derive(Debug, Clone)]
pub(crate) struct CreateCharacterArgs {
    pub id: CharacterId,
    pub name: String,
}

impl CharacterApi {
    /// Creates a new character entity, persists it to the database, and returns
    /// summary information about it.
    ///
    /// If a character with the given ID already exists, its info is returned
    /// without creating a duplicate (upsert semantics).
    pub async fn create(
        &self,
        id: CharacterId,
        name: impl Into<String>,
    ) -> ApiResult<CharacterInfo> {
        let name = name.into();
        self.0
            .schedule(move |task| async move {
                let args = CreateCharacterArgs { id, name };
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
    characters: Query<(
        &CharacterId,
        &CharacterName,
        &AssetIdComponent,
        &CharacterState,
    )>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult<CharacterInfo> {
    if let Some(entity) = registry.get(&args.id) {
        return build_info_from_entity(entity, &characters);
    }

    persist_character(&db, &args)?;

    let info = CharacterInfo {
        id: args.id.to_string(),
        name: args.name.clone(),
        state: CharacterState::default().0.clone(),
        has_vrm: false,
    };
    commands.spawn((
        Character,
        args.id,
        CharacterName(args.name),
        Name::new(String::new()),
        CharacterState::default(),
        Persona::default(),
    ));
    Ok(info)
}

fn build_info_from_entity(
    entity: Entity,
    characters: &Query<(
        &CharacterId,
        &CharacterName,
        &AssetIdComponent,
        &CharacterState,
    )>,
) -> ApiResult<CharacterInfo> {
    let (id, name, asset_id, state) = characters
        .get(entity)
        .map_err(|_| ApiError::CharacterNotFound(entity.to_string()))?;
    Ok(CharacterInfo {
        id: id.to_string(),
        name: name.0.clone(),
        state: state.0.clone(),
        has_vrm: false,
    })
}

/// Inserts the character row into the database.
fn persist_character(db: &PrefsDatabase, args: &CreateCharacterArgs) -> ApiResult<()> {
    let persona_json =
        serde_json::to_string(&Persona::default()).unwrap_or_else(|_| "{}".to_string());
    CharactersTable::new(db)
        .create(&args.id, &args.name, &persona_json, "{}")
        .map_err(|e| ApiError::Sql(e.to_string()))
}
