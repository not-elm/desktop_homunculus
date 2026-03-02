use crate::entities::EntitiesApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{Coordinate, GlobalViewport};
use serde::{Deserialize, Serialize};

/// Target position for an entity move operation.
///
/// - `World`: Set position in Bevy world coordinates. The `z` field is optional;
///   when omitted, the entity's current z-position is preserved.
/// - `Viewport`: Set position using global viewport (screen) coordinates,
///   which are converted to world coordinates using the active camera.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(tag = "type")]
pub enum MoveTarget {
    #[serde(rename = "world")]
    World {
        #[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
        position: Vec2,
        z: Option<f32>,
    },
    #[serde(rename = "viewport")]
    Viewport {
        #[cfg_attr(feature = "openapi", schema(value_type = [f32; 2]))]
        position: Vec2,
    },
}

impl EntitiesApi {
    pub async fn move_to(&self, entity: Entity, target: MoveTarget) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(move_to_system).with((entity, target)))
                    .await
            })
            .await?
    }
}

fn move_to_system(
    In((entity, target)): In<(Entity, MoveTarget)>,
    coordinate: Coordinate,
    mut transforms: Query<&mut Transform>,
) -> ApiResult {
    let Ok(mut tf) = transforms.get_mut(entity) else {
        return Err(ApiError::EntityNotFound);
    };

    match target {
        MoveTarget::World { position, z } => {
            tf.translation.x = position.x;
            tf.translation.y = position.y;
            if let Some(z) = z {
                tf.translation.z = z;
            }
        }
        MoveTarget::Viewport { position } => {
            let global_viewport = GlobalViewport(position);
            let Some(world_pos) = coordinate.to_world_2d_by_global(global_viewport) else {
                return Err(ApiError::FailedToWorldPosition);
            };
            tf.translation.x = world_pos.x;
            tf.translation.y = world_pos.y;
        }
    }
    Ok(())
}
