use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::Vrm;
use homunculus_core::prelude::VrmMetadata;

impl VrmApi {
    pub async fn fetch_all(&self) -> ApiResult<Vec<VrmMetadata>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(all_vrms)).await })
            .await
    }
}

fn all_vrms(vrms: Query<(Entity, &Name), With<Vrm>>) -> Vec<VrmMetadata> {
    vrms.iter()
        .map(|(entity, name)| VrmMetadata {
            name: name.to_string(),
            entity,
        })
        .collect()
}
