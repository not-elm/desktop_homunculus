use crate::api;
use crate::error::ApiResult;
use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use bevy_vrm1::prelude::{PlayVrma, StopVrma, Vrma, VrmaAnimationPlayers};
use bevy_vrm1::vrma::VrmaDuration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

api!(VrmAnimationApi);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct VrmaState {
    pub playing: bool,
    pub repeat: String,
    pub speed: f32,
    pub elapsed_secs: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct VrmaInfo {
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub entity: Entity,
    pub name: String,
    pub playing: bool,
}

impl VrmAnimationApi {
    /// Plays a VRM animation.
    ///
    /// If `wait_finish` is set to `true`, it will wait until the animation finishes.
    pub async fn play(&self, args: PlayVrma, wait_finish: bool) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                let vrma = args.vrma;
                task.will(Update, once::run(play).with(args)).await;

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

    /// Returns the current state of a VRMA animation.
    pub async fn state(&self, vrma: Entity) -> ApiResult<VrmaState> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_vrma_state).with(vrma))
                    .await
            })
            .await
    }

    /// Sets the playback speed for all active animations in a VRMA.
    pub async fn set_speed(&self, vrma: Entity, speed: f32) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_vrma_speed).with((vrma, speed)))
                    .await;
            })
            .await
    }

    /// Lists all VRMA animations under a VRM entity.
    pub async fn list_all(&self, vrm: Entity) -> ApiResult<Vec<VrmaInfo>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(list_all_vrma).with(vrm)).await
            })
            .await
    }
}

fn play(In(event): In<PlayVrma>, mut commands: Commands) {
    commands.trigger(event);
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
    commands.trigger(StopVrma { entity: vrma });
}

fn get_vrma_state(
    In(vrma): In<Entity>,
    vrma_players: Query<&VrmaAnimationPlayers>,
    players: Query<&AnimationPlayer>,
) -> VrmaState {
    let Ok(animation_players) = vrma_players.get(vrma) else {
        return VrmaState {
            playing: false,
            repeat: "never".to_string(),
            speed: 1.0,
            elapsed_secs: 0.0,
        };
    };

    let mut playing = false;
    let mut repeat_str = "never".to_string();
    let mut speed = 1.0;
    let mut elapsed = 0.0f32;

    for &player_entity in animation_players.0.iter() {
        if let Ok(player) = players.get(player_entity)
            && let Some((_, anim)) = player.playing_animations().next()
        {
            playing = !anim.is_finished();
            speed = anim.speed();
            elapsed = anim.elapsed();
            repeat_str = match anim.repeat_mode() {
                RepeatAnimation::Never => "never".to_string(),
                RepeatAnimation::Count(n) => format!("count:{n}"),
                RepeatAnimation::Forever => "forever".to_string(),
            };
        }
    }

    VrmaState {
        playing,
        repeat: repeat_str,
        speed,
        elapsed_secs: elapsed,
    }
}

fn set_vrma_speed(
    In((vrma, speed)): In<(Entity, f32)>,
    vrma_players: Query<&VrmaAnimationPlayers>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_players) = vrma_players.get(vrma) {
        for &player_entity in animation_players.0.iter() {
            if let Ok(mut player) = players.get_mut(player_entity) {
                for (_, anim) in player.playing_animations_mut() {
                    anim.set_speed(speed);
                }
            }
        }
    }
}

fn list_all_vrma(
    In(vrm): In<Entity>,
    children: Query<&Children>,
    vrma_query: Query<(Entity, &Name, &VrmaAnimationPlayers), With<Vrma>>,
    players: Query<&AnimationPlayer>,
) -> Vec<VrmaInfo> {
    let mut result = Vec::new();
    let Ok(vrm_children) = children.get(vrm) else {
        return result;
    };
    for child in vrm_children.iter() {
        if let Ok((entity, name, animation_players)) = vrma_query.get(child) {
            let playing = animation_players
                .0
                .iter()
                .any(|&pe| players.get(pe).map(|p| !p.all_finished()).unwrap_or(false));
            result.push(VrmaInfo {
                entity,
                name: name.to_string(),
                playing,
            });
        }
    }
    result
}
