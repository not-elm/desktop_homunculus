use crate::cameras::CameraApi;
use crate::error::{ApiError, ApiResult};
use bevy::math::Vec2;
use bevy::prelude::{In, Update};
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{Coordinate, GlobalViewport};
use serde::{Deserialize, Serialize};

impl CameraApi {
    pub async fn world_2d(&self, args: OptionalGlobalViewport) -> ApiResult<Vec2> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(world_pos_by_global_viewport).with(args))
                    .await
            })
            .await?
    }
}

/// Represents an optional global viewport with x and y coordinates.
/// The unspecified values will be replaced with 0.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalGlobalViewport {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

fn world_pos_by_global_viewport(
    In(query): In<OptionalGlobalViewport>,
    coordinate: Coordinate,
) -> ApiResult<Vec2> {
    coordinate
        .to_world_2d_by_global(GlobalViewport(Vec2::new(
            query.x.unwrap_or_default(),
            query.y.unwrap_or_default(),
        )))
        .ok_or(ApiError::FailedToWorldPosition)
}
