use crate::error::{ApiResult, ApiResultExt};
use crate::prelude::initialized;
use crate::vrm::VrmApi;
use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, Entity, In, Res, Transform, Update};
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{Cameras, LookAt, VrmHandle};
use homunculus_core::prelude::{ModModuleSource, VrmState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VrmSpawnArgs {
    pub asset: ModModuleSource,
    pub transform: Option<TransformArgs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformArgs {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
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
    asset_server: Res<AssetServer>,
    cameras: Cameras,
) -> Option<Entity> {
    let entity = commands
        .spawn((
            spawn_vrm.asset.clone(),
            VrmHandle(asset_server.load(spawn_vrm.asset.to_string())),
            spawn_vrm
                .transform
                .map(|t| t.into_transform())
                .unwrap_or_default(),
            VrmState::default(),
            cameras.all_layers(),
            LookAt::Cursor { camera: None },
        ))
        .id();
    Some(entity)
}
