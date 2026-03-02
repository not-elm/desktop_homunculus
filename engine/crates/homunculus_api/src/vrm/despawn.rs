use crate::error::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::{Commands, Entity, In, Update};
use bevy_flurx::prelude::*;

impl VrmApi {
    pub async fn despawn(&self, vrm: Entity) -> ApiResult {
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(despawn).with(vrm)).await },
            )
            .await
    }
}

fn despawn(In(vrm): In<Entity>, mut commands: Commands) {
    commands.entity(vrm).try_despawn();
}
