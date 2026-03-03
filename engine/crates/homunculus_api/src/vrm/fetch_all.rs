use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::Vrm;

impl VrmApi {
    pub async fn fetch_all(&self) -> ApiResult<Vec<(Entity, String)>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(all_vrms)).await })
            .await
    }
}

fn all_vrms(vrms: Query<(Entity, &Name), With<Vrm>>) -> Vec<(Entity, String)> {
    vrms.iter()
        .map(|(entity, name)| (entity, name.to_string()))
        .collect()
}
