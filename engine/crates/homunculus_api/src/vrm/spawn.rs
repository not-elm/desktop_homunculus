use crate::error::{ApiResult, ApiResultExt};
use crate::prelude::initialized;
use crate::vrm::VrmApi;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, Entity, In, NonSend, Transform, Update, warn};
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{BodyTracking, Cameras, LookAt, VrmHandle};
use homunculus_core::prelude::{AssetId, AssetIdComponent, AssetResolver, AvatarState, Persona};
use homunculus_prefs::prelude::{PrefsDatabase, PrefsKeys};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VrmSpawnArgs {
    pub asset: AssetId,
    pub transform: Option<TransformArgs>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Object>))]
    pub persona: Option<Persona>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct TransformArgs {
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 3]>))]
    pub translation: Option<Vec3>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 4]>))]
    pub rotation: Option<Quat>,
    #[cfg_attr(feature = "openapi", schema(value_type = Option<[f32; 3]>))]
    pub scale: Option<Vec3>,
}

impl TransformArgs {
    pub fn into_transform(self) -> Transform {
        Transform {
            translation: self.translation.unwrap_or_default(),
            rotation: self.rotation.unwrap_or(Quat::IDENTITY),
            scale: self.scale.unwrap_or(Vec3::ONE),
        }
    }
}

impl VrmApi {
    pub async fn spawn(&self, spawn_vrm: VrmSpawnArgs) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                let entity = task.will(Update, once::run(spawn).with(spawn_vrm)).await?;
                task.will(Update, wait::until(initialized).with(entity))
                    .await;
                Some(entity)
            })
            .await
            .error_if_notfound()
    }
}

fn spawn(
    In(spawn_vrm): In<VrmSpawnArgs>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
    cameras: Cameras,
    prefs: NonSend<PrefsDatabase>,
) -> Option<Entity> {
    let handle = asset_resolver.load(&spawn_vrm.asset).ok()?;

    let persona = if let Some(p) = spawn_vrm.persona {
        let key = PrefsKeys::persona(spawn_vrm.asset.as_ref());
        if let Err(e) = prefs.save_as(&key, &p) {
            warn!("Failed to save persona preference: {e}");
        }
        p
    } else {
        let key = PrefsKeys::persona(spawn_vrm.asset.as_ref());
        prefs
            .load_as::<Persona>(&key)
            .ok()
            .flatten()
            .unwrap_or_default()
    };

    let entity = commands
        .spawn((
            AssetIdComponent(spawn_vrm.asset),
            VrmHandle(handle),
            spawn_vrm
                .transform
                .map(|t| t.into_transform())
                .unwrap_or_default(),
            AvatarState::default(),
            cameras.all_layers(),
            LookAt::Cursor,
            BodyTracking::default(),
            persona,
        ))
        .id();
    Some(entity)
}
