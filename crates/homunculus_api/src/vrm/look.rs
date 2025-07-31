use crate::prelude::ApiResult;
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{LeftEyeBoneEntity, LookAt, RightEyeBoneEntity};
use bevy_vrm1::vrm::BoneRestTransform;

impl VrmApi {
    /// Disables look-at control for the specified VRM entity.
    pub async fn unlook(&self, vrm_entity: Entity) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(unlook).with(vrm_entity)).await;
            })
            .await
    }

    /// Enables look-at control for the specified VRM entity, targeting another entity.
    pub async fn look_at_target(&self, vrm: Entity, target: Entity) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(look_at_target).with((vrm, target)))
                    .await;
            })
            .await
    }

    /// Enables look-at control for the specified VRM entity, making it look at the cursor.
    pub async fn look_at_cursor(&self, vrm: Entity) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(look_at_cursor).with(vrm)).await;
            })
            .await
    }
}

fn unlook(
    In(vrm): In<Entity>,
    mut commands: Commands,
    vrms: Query<(&LeftEyeBoneEntity, &RightEyeBoneEntity)>,
    rests: Query<&BoneRestTransform>,
) {
    commands.entity(vrm).try_remove::<LookAt>();
    if let Ok((left_eye, right_eye)) = vrms.get(vrm)
        && let Ok(left_eye_rest) = rests.get(left_eye.0)
        && let Ok(right_eye_rest) = rests.get(right_eye.0)
    {
        commands.entity(left_eye.0).try_insert(left_eye_rest.0);
        commands.entity(right_eye.0).try_insert(right_eye_rest.0);
    }
}

fn look_at_target(In((vrm, target)): In<(Entity, Entity)>, mut commands: Commands) {
    commands.entity(vrm).try_insert(LookAt::Target(target));
}

fn look_at_cursor(In(vrm): In<Entity>, mut commands: Commands) {
    commands
        .entity(vrm)
        .try_insert(LookAt::Cursor { camera: None });
}
