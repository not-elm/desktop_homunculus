use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::{VrmApi, initialized};
use bevy::prelude::*;
use bevy_flurx::prelude::{once, wait};
use bevy_vrm1::vrma::VrmaHandle;
use homunculus_core::prelude::ModModuleSource;

impl VrmApi {
    /// Fetches the VRMA from the asset server if it exists, or spawns a new entity with the VRMA asset if it does not.
    pub async fn vrma(&self, vrm_entity: Entity, vrma: ModModuleSource) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                let vrma_entity = task
                    .will(Update, once::run(fetch_vrma).with((vrm_entity, vrma)))
                    .await?;
                task.will(Update, wait::until(initialized).with(vrma_entity))
                    .await;
                Some(vrma_entity)
            })
            .await
            .error_if_notfound()
    }
}

fn fetch_vrma(
    In((vrm_entity, asset_path)): In<(Entity, ModModuleSource)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities: Query<Entity>,
    vrms: Query<&Children>,
    vrmas: Query<(Entity, &ModModuleSource)>,
) -> Option<Entity> {
    if !entities.contains(vrm_entity) {
        return None;
    }
    if let Ok(children) = vrms.get(vrm_entity)
        && let Some(vrma_entity) = children
            .iter()
            .flat_map(|c| vrmas.get(c).ok())
            .find_map(|(e, id)| (id == &asset_path).then_some(e))
    {
        return Some(vrma_entity);
    }
    let vrma_entity = commands
        .spawn((
            asset_path.clone(),
            VrmaHandle(asset_server.load(asset_path.to_string())),
        ))
        .id();
    commands.entity(vrm_entity).add_child(vrma_entity);
    Some(vrma_entity)
}
