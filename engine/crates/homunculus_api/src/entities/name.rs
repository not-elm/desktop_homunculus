use crate::entities::EntitiesApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::*;

impl EntitiesApi {
    pub async fn name(&self, entity: Entity) -> ApiResult<String> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_name).with(entity)).await
            })
            .await?
    }
}

fn get_name(In(entity): In<Entity>, entities: Query<&Name>) -> ApiResult<String> {
    entities
        .get(entity)
        .map(|name| name.to_string())
        .map_err(|_| ApiError::MissingName(entity))
}
