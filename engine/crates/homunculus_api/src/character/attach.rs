use crate::character::CharacterApi;
use crate::error::{ApiError, ApiResult};
use crate::vrm::initialized;
use bevy::prelude::*;
use bevy_flurx::action::delay;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, RequestDetachVrm, VrmHandle};
use bevy_vrm1::vrm::Vrm;
use homunculus_core::prelude::{
    AssetId, AssetIdComponent, AssetResolver, CharacterId, CharacterRegistry,
};
use homunculus_prefs::characters::CharactersTable;
use homunculus_prefs::prelude::PrefsDatabase;

/// Arguments for attaching a VRM to a character.
#[derive(Debug, Clone)]
struct AttachVrmArgs {
    id: CharacterId,
    asset_id: AssetId,
}

impl CharacterApi {
    /// Loads a VRM model and attaches it to an existing character entity.
    ///
    /// Waits for the VRM to finish initialization before returning. After
    /// initialization, the character's display name is restored (since VRM
    /// loading overwrites the Bevy `Name` component).
    pub async fn attach_vrm(&self, id: CharacterId, asset_id: AssetId) -> ApiResult<Entity> {
        let args = AttachVrmArgs { id, asset_id };
        self.0
            .schedule(move |task| async move {
                let entity = task
                    .will(Update, once::run(begin_attach).with(args))
                    .await?;
                let result = task
                    .will(
                        Update,
                        wait::either(
                            wait::until(initialized).with(entity),
                            delay::frames().with(600),
                        ),
                    )
                    .await;
                if result.is_right() {
                    return Err(ApiError::VrmInitTimeout(entity));
                }
                Ok(entity)
            })
            .await?
    }

    /// Detaches the VRM model from a character entity.
    ///
    /// The character entity itself remains alive; only the VRM hierarchy and
    /// related components are removed.
    pub async fn detach_vrm(&self, id: CharacterId) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(begin_detach).with(id)).await
            })
            .await?
    }
}

fn begin_attach(
    In(args): In<AttachVrmArgs>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    asset_resolver: AssetResolver,
    cameras: Cameras,
) -> ApiResult<Entity> {
    let entity = registry
        .get(&args.id)
        .ok_or_else(|| ApiError::CharacterNotFound(args.id.to_string()))?;

    let handle = asset_resolver
        .load(&args.asset_id)
        .map_err(|_| ApiError::AssetNotFound(args.asset_id.clone()))?;

    commands.entity(entity).try_insert((
        VrmHandle(handle),
        AssetIdComponent(args.asset_id.clone()),
        LookAt::Cursor,
        BodyTracking::default(),
        cameras.all_layers(),
    ));

    Ok(entity)
}

fn begin_detach(
    In(id): In<CharacterId>,
    mut commands: Commands,
    registry: Res<CharacterRegistry>,
    vrm_check: Query<(), With<Vrm>>,
) -> ApiResult {
    let entity = registry
        .get(&id)
        .ok_or_else(|| ApiError::CharacterNotFound(id.to_string()))?;

    if vrm_check.get(entity).is_err() {
        return Err(ApiError::VrmNotAttached(id.to_string()));
    }

    commands.entity(entity).trigger(RequestDetachVrm);
    // LookAt and BodyTracking are app-level components inserted by attach_vrm,
    // so homunculus is responsible for their removal (Design Decision #2).
    commands
        .entity(entity)
        .remove::<LookAt>()
        .remove::<BodyTracking>();

    Ok(())
}
