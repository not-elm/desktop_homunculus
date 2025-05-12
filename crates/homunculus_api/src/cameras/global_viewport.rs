use crate::cameras::CameraApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::{In, Update, Vec3};
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{Coordinate, GlobalViewport};
use serde::{Deserialize, Serialize};

impl CameraApi {
    pub async fn global_viewport(&self, args: GlobalViewportArgs) -> ApiResult<GlobalViewport> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(global_viewport_by_world).with(args))
                    .await
            })
            .await?
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct GlobalViewportArgs {
    x: Option<f32>,
    y: Option<f32>,
    z: Option<f32>,
}

fn global_viewport_by_world(
    In(world_pos): In<GlobalViewportArgs>,
    coordinate: Coordinate,
) -> ApiResult<GlobalViewport> {
    let world_pos = Vec3::new(
        world_pos.x.unwrap_or_default(),
        world_pos.y.unwrap_or_default(),
        world_pos.z.unwrap_or_default(),
    );
    coordinate
        .to_global_by_world(world_pos)
        .ok_or(ApiError::FailedToGlobalViewport)
}
