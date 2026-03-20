use crate::avatar::AvatarApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{
    AssetId, AssetIdComponent, Avatar, AvatarId, AvatarName, AvatarRegistry, AvatarState, Persona,
};
use homunculus_prefs::avatar_repo::AvatarRepo;
use homunculus_prefs::prelude::PrefsDatabase;

/// Arguments for creating a new avatar.
#[derive(Debug, Clone)]
pub(crate) struct CreateAvatarArgs {
    pub id: AvatarId,
    pub asset_id: AssetId,
    pub name: String,
    pub ensure: bool,
}

impl AvatarApi {
    /// Creates a new avatar entity and persists it to the database.
    ///
    /// When `ensure` is true and an avatar with the given ID already exists,
    /// the existing entity is returned instead of raising an error.
    pub async fn create(
        &self,
        id: AvatarId,
        asset_id: AssetId,
        name: String,
        ensure: bool,
    ) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                let args = CreateAvatarArgs {
                    id,
                    asset_id,
                    name,
                    ensure,
                };
                task.will(Update, once::run(create_avatar).with(args)).await
            })
            .await?
    }
}

fn create_avatar(
    In(args): In<CreateAvatarArgs>,
    mut commands: Commands,
    registry: Res<AvatarRegistry>,
    db: NonSend<PrefsDatabase>,
) -> ApiResult<Entity> {
    if let Some(entity) = registry.get(&args.id) {
        if args.ensure {
            return Ok(entity);
        }
        return Err(ApiError::AvatarAlreadyExists(args.id.to_string()));
    }

    persist_avatar(&db, &args)?;

    let entity = commands
        .spawn((
            Avatar,
            args.id,
            AvatarName(args.name),
            Name::new(String::new()),
            AssetIdComponent(args.asset_id),
            AvatarState::default(),
            Persona::default(),
        ))
        .id();
    Ok(entity)
}

/// Inserts the avatar row into the database.
fn persist_avatar(db: &PrefsDatabase, args: &CreateAvatarArgs) -> ApiResult<()> {
    let persona_json =
        serde_json::to_string(&Persona::default()).unwrap_or_else(|_| "{}".to_string());
    AvatarRepo::new(db)
        .create(
            &args.id,
            args.asset_id.as_ref(),
            &args.name,
            &persona_json,
            "{}",
        )
        .map_err(|e| ApiError::Sql(e.to_string()))
}
