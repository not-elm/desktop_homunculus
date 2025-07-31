use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::Initialized;
use bevy_vrm1::vrm::Vrm;

impl VrmApi {
    pub async fn wait_load_by_name(&self, name: String) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, wait::output(loaded).with(name)).await
            })
            .await
    }
}

fn loaded(
    In(target): In<String>,
    vrms: Query<(Entity, &Name), (With<Initialized>, With<Vrm>)>,
) -> Option<Entity> {
    vrms.iter()
        .find_map(|(entity, name)| (name.as_str() == target.as_str()).then_some(entity))
}
