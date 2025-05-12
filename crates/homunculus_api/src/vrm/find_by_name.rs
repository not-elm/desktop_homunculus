use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::VrmApi;
use bevy::prelude::{Entity, In, Name, Query, Update, With};
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::Vrm;

impl VrmApi {
    pub async fn find_by_name(&self, vrm_name: String) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(fetch_entity).with(vrm_name))
                    .await
                    .map(Entity::from_bits)
            })
            .await
            .error_if_notfound()
    }
}

fn fetch_entity(In(name): In<String>, vrms: Query<(Entity, &Name), With<Vrm>>) -> Option<u64> {
    vrms.iter()
        .find_map(|(entity, n)| (n.as_str() == name).then_some(entity.to_bits()))
}
