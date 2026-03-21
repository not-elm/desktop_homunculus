use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::ChildSearcher;
use bevy_vrm1::vrm::VrmBone;
use homunculus_core::prelude::{CharacterId, CharacterRegistry};

impl VrmApi {
    pub async fn bone(&self, character_id: CharacterId, bone_name: VrmBone) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(find_bone).with((character_id, bone_name)))
                    .await
            })
            .await
            .error_if_notfound()
    }
}

fn find_bone(
    In((id, bone_name)): In<(CharacterId, VrmBone)>,
    searcher: ChildSearcher,
    registry: Res<CharacterRegistry>,
) -> Option<Entity> {
    let entity = registry.get(&id)?;
    searcher.find_by_bone_name(entity, &bone_name)
}
