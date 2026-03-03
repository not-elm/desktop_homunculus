use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Coordinate, GlobalViewport};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct PositionResponse {
    #[cfg_attr(feature = "openapi", schema(value_type = Option<Object>))]
    pub global_viewport: Option<GlobalViewport>,
    #[cfg_attr(feature = "openapi", schema(value_type = [f32; 3]))]
    pub world: Vec3,
}

impl VrmApi {
    pub async fn position(&self, vrm: Entity) -> ApiResult<PositionResponse> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_position).with(vrm))
                    .await
                    .expect("Failed to get VRM position")
            })
            .await
    }
}

fn get_position(
    In(entity): In<Entity>,
    transforms: Query<&Transform>,
    coordinate: Coordinate,
) -> Option<PositionResponse> {
    let transform = transforms.get(entity).ok()?;
    let global_viewport = coordinate.to_global_by_world(transform.translation);
    Some(PositionResponse {
        global_viewport,
        world: transform.translation,
    })
}
