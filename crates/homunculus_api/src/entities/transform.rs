use crate::entities::EntitiesApi;
use crate::error::ApiResult;
use crate::prelude::ApiError;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use serde::{Deserialize, Serialize};

impl EntitiesApi {
    pub async fn transform(&self, entity: Entity) -> ApiResult<Transform> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(fetch_transform).with(entity))
                    .await
            })
            .await?
    }

    pub async fn set_transform(
        &self,
        entity: Entity,
        body: OptionalTransform,
    ) -> ApiResult<Option<Transform>> {
        self.0
            .schedule(move |task| async move {
                // `Update` doesn't update when sitting
                task.will(PreUpdate, once::run(set_transform).with((entity, body)))
                    .await
            })
            .await
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OptionalTransform {
    pub translation: Option<Vec3>,
    pub rotation: Option<Quat>,
    pub scale: Option<Vec3>,
}

fn fetch_transform(In(entity): In<Entity>, transforms: Query<&Transform>) -> ApiResult<Transform> {
    transforms
        .get(entity)
        .copied()
        .map_err(|_| ApiError::EntityNotfound)
}

fn set_transform(
    In((entity, body)): In<(Entity, OptionalTransform)>,
    mut transforms: Query<&mut Transform>,
) -> Option<Transform> {
    let mut transform = transforms.get_mut(entity).ok()?;
    if let Some(translation) = body.translation {
        transform.translation = translation;
    }
    if let Some(rotation) = body.rotation {
        transform.rotation = rotation;
    }
    if let Some(scale) = body.scale {
        transform.scale = scale;
    }
    Some(*transform)
}
