use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::{VrmApi, initialized};
use bevy::prelude::*;
use bevy_flurx::prelude::{once, wait};
use bevy_vrm1::vrma::VrmaHandle;
use homunculus_core::prelude::{AssetId, AssetIdComponent, AssetResolver};

impl VrmApi {
    /// Fetches the VRMA from the asset server if it exists, or spawns a new entity with the VRMA asset if it does not.
    pub async fn vrma(&self, vrm_entity: Entity, asset_id: AssetId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                let vrma_entity = task
                    .will(Update, once::run(fetch_vrma).with((vrm_entity, asset_id)))
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
    In((vrm_entity, asset_id)): In<(Entity, AssetId)>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
    entities: Query<Entity>,
    vrms: Query<&Children>,
    vrmas: Query<(Entity, &AssetIdComponent)>,
) -> Option<Entity> {
    if !entities.contains(vrm_entity) {
        return None;
    }
    if let Ok(children) = vrms.get(vrm_entity)
        && let Some(vrma_entity) = children
            .iter()
            .flat_map(|c| vrmas.get(c).ok())
            .find_map(|(e, id)| (id.0 == asset_id).then_some(e))
    {
        return Some(vrma_entity);
    }
    let handle = asset_resolver.load(&asset_id).ok()?;
    let vrma_entity = commands
        .spawn((AssetIdComponent(asset_id), VrmaHandle(handle)))
        .id();
    commands.entity(vrm_entity).add_child(vrma_entity);
    Some(vrma_entity)
}
