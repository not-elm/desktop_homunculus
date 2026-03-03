use crate::error::{ApiResult, ApiResultExt};
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::ChildSearcher;
use bevy_vrm1::vrm::VrmBone;

impl VrmApi {
    pub async fn bone(&self, root: Entity, bone_name: VrmBone) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(find_bone).with((root, bone_name)))
                    .await
            })
            .await
            .error_if_notfound()
    }
}

fn find_bone(
    In((root, bone_name)): In<(Entity, VrmBone)>,
    searcher: ChildSearcher,
) -> Option<Entity> {
    searcher.find_by_bone_name(root, &bone_name)
}
