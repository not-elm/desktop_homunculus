use crate::api;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{PlayVrma, StopVrma};
use bevy_vrm1::vrma::VrmaDuration;
use std::time::Duration;

api!(VrmAnimationApi);

impl VrmAnimationApi {
    /// Plays a VRM animation.
    ///
    /// If `wait_finish` is set to `true`, it will wait until the animation finishes.
    pub async fn play(&self, vrma: Entity, wait_finish: bool, args: PlayVrma) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(play).with((vrma, args))).await;

                let wait_animation_finished =
                    once::run(wait_animation_finished).with((vrma, wait_finish));
                if let Some(duration) = task.will(Update, wait_animation_finished).await {
                    task.will(Update, delay::time().with(duration)).await;
                }
            })
            .await
    }

    /// Stops a VRM animation immediately.
    pub async fn stop(&self, vrma: Entity) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(stop).with(vrma)).await;
            })
            .await
    }
}

fn play(In((vrma, args)): In<(Entity, PlayVrma)>, mut commands: Commands) {
    commands.entity(vrma).trigger(args);
}

fn wait_animation_finished(
    In((vrma, wait_finish)): In<(Entity, bool)>,
    vrmas: Query<&VrmaDuration>,
) -> Option<Duration> {
    if !wait_finish {
        return None;
    }
    vrmas.get(vrma).ok().map(|duration| duration.0)
}

fn stop(In(vrma): In<Entity>, mut commands: Commands) {
    commands.entity(vrma).trigger(StopVrma);
}
