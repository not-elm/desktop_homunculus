use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use bevy::animation::RepeatAnimation;
use bevy_vrm1::prelude::PlayVrma;
use homunculus_api::prelude::VrmAnimationApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayBody {
    /// Duration in seconds for the transition of the animation.
    #[serde(rename = "transitionSecs")]
    pub transition_secs: Option<f64>,
    /// Repetition behavior of an animation.
    pub repeat: Option<Repeat>,
    /// If true, the API will wait until the animation finishes.
    #[serde(rename = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Repeat {
    Forever,
    Never,
    Count { count: u32 },
}

/// Plays a VRM animation.
///
/// ## Path
///
/// `POST /vrma/:vrma/play`
pub async fn play(
    State(api): State<VrmAnimationApi>,
    EntityId(vrma): EntityId,
    Json(body): Json<PlayBody>,
) -> HttpResult {
    let mut args = PlayVrma::default();
    if let Some(transition_secs) = body.transition_secs {
        args.transition_duration = Duration::from_secs_f64(transition_secs);
    }
    if let Some(repeat) = body.repeat {
        args.repeat = match repeat {
            Repeat::Forever => RepeatAnimation::Forever,
            Repeat::Never => RepeatAnimation::Never,
            Repeat::Count { count } => RepeatAnimation::Count(count),
        };
    }
    api.play(vrma, body.wait_for_completion.unwrap_or_default(), args)
        .await
        .into_http_result()
}

/// Stops a VRM animation immediately.
///
/// ## Path
///
/// `POST /vrma/:vrma/stop`
pub async fn stop(State(api): State<VrmAnimationApi>, EntityId(vrma): EntityId) -> HttpResult {
    api.stop(vrma).await.into_http_result()
}
