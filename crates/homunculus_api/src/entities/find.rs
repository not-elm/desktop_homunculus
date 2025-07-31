use crate::error::ApiError;
use crate::prelude::{ApiResult, EntitiesApi};
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use bevy_vrm1::prelude::ChildSearcher;

impl EntitiesApi {
    /// Finds an entity by its name.
    ///
    /// If `root` entity is specified, it will search recursively from that entity's children.
    pub async fn find_by_name(&self, name: Name, root: Option<Entity>) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(find_by_name).with((name, root)))
                    .await
            })
            .await?
    }
}

fn find_by_name(
    In((name, root)): In<(Name, Option<Entity>)>,
    entities: Query<(Entity, &Name)>,
    child_searcher: ChildSearcher,
) -> ApiResult<Entity> {
    match root {
        Some(root) => {
            if !entities.contains(root) {
                return Err(ApiError::NotFoundEntityByName(name));
            }
            child_searcher
                .find_by_name(root, &name)
                .ok_or(ApiError::NotFoundEntityByName(name))
        }
        None => entities
            .iter()
            .find(|(_, entity_name)| entity_name == &&name)
            .map(|(entity, _)| entity)
            .ok_or(ApiError::NotFoundEntityByName(name)),
    }
}
