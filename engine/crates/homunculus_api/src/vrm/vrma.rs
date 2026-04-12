use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::{VrmApi, initialized};
use bevy::prelude::*;
use bevy_flurx::prelude::{once, wait};
use bevy_vrm1::vrma::VrmAnimationNodeIndex;
use bevy_vrm1::vrma::VrmaHandle;
use bevy_vrm1::vrma::animation::animation_graph::NeedsBake;
use homunculus_core::prelude::{AssetId, AssetIdComponent, AssetResolver};

impl VrmApi {
    /// Fetches the VRMA from the asset server if it exists, or spawns a new entity with the VRMA asset if it does not.
    ///
    /// Waits for `Initialized`, then waits until the animation graph is fully
    /// constructed (`VrmAnimationNodeIndex` present) and clips are baked
    /// (`NeedsBake` absent). This ensures that `PlayVrma` triggers will find
    /// the required components.
    pub async fn vrma(&self, vrm_entity: Entity, asset_id: AssetId) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                info!("[vrma] fetch_vrma: vrm={vrm_entity}, asset={asset_id}");
                let vrma_entity = task
                    .will(Update, once::run(fetch_vrma).with((vrm_entity, asset_id)))
                    .await?;
                info!("[vrma] fetch_vrma returned: vrma_entity={vrma_entity}");
                task.will(Update, wait::until(initialized).with(vrma_entity))
                    .await;
                info!("[vrma] Initialized on {vrma_entity}. Waiting for animation readiness...");
                task.will(Update, wait::until(animation_ready).with(vrma_entity))
                    .await;
                info!("[vrma] Animation ready on {vrma_entity}. Returning.");
                Some(vrma_entity)
            })
            .await
            .error_if_notfound()
    }
}

/// Returns true when the VRMA animation pipeline is complete:
/// - `VrmAnimationNodeIndex` is present (animation graph constructed)
/// - `NeedsBake` is absent (clip baking finished, or was never needed)
fn animation_ready(
    In(entity): In<Entity>,
    nodes: Query<&VrmAnimationNodeIndex>,
    baking: Query<(), With<NeedsBake>>,
) -> bool {
    nodes.contains(entity) && !baking.contains(entity)
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
        info!("[vrma] fetch_vrma: vrm_entity {vrm_entity} not found");
        return None;
    }
    if let Ok(children) = vrms.get(vrm_entity)
        && let Some(vrma_entity) = children
            .iter()
            .flat_map(|c| vrmas.get(c).ok())
            .find_map(|(e, id)| (id.0 == asset_id).then_some(e))
    {
        info!("[vrma] fetch_vrma: found existing VRMA {vrma_entity} for {asset_id}");
        return Some(vrma_entity);
    }
    let handle = asset_resolver.load(&asset_id).ok()?;
    let vrma_entity = commands
        .spawn((AssetIdComponent(asset_id.clone()), VrmaHandle(handle)))
        .id();
    commands.entity(vrm_entity).add_child(vrma_entity);
    info!("[vrma] fetch_vrma: spawned new VRMA {vrma_entity} for {asset_id}");
    Some(vrma_entity)
}
