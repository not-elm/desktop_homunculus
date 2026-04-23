use axum::Json;
use axum::extract::State;
use bevy::animation::RepeatAnimation;
use bevy_vrm1::prelude::PlayVrma;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{VrmAnimationApi, VrmaInfo};
use homunculus_api::vrm::VrmApi;
use homunculus_core::prelude::AssetId;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use utoipa::ToSchema;

use crate::route::AssetRequest;
use crate::route::persona::SpawnedPersonaPath;

/// Request body for playing a VRMA animation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PlayBody {
    /// Asset ID for the VRMA animation to play.
    pub asset: AssetId,
    /// Duration in seconds for the transition of the animation.
    #[serde(rename = "transitionSecs")]
    pub transition_secs: Option<f64>,
    /// Repetition behavior of an animation.
    pub repeat: Option<Repeat>,
    /// If true, the API will wait until the animation finishes.
    #[serde(rename = "waitForCompletion")]
    pub wait_for_completion: Option<bool>,
    /// If true, resets SpringBone velocities to prevent bouncing during animation transitions.
    #[serde(rename = "resetSpringBones")]
    pub reset_spring_bones: Option<bool>,
}

/// Repetition behavior of a VRMA animation.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Repeat {
    /// Loop forever.
    Forever,
    /// Play once and stop.
    Never,
    /// Repeat a fixed number of times.
    Count {
        /// Number of repetitions.
        count: u32,
    },
}

/// List all VRMA animations under a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/vrma",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "List of VRMA animations", body = Vec<VrmaInfo>),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn get_vrma(
    State(api): State<VrmAnimationApi>,
    path: SpawnedPersonaPath,
) -> HttpResult<Vec<VrmaInfo>> {
    api.list_all(path.entity).await.into_http_result()
}

/// Play a VRMA animation on a persona's VRM.
#[utoipa::path(
    post,
    path = "/vrm/vrma/play",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = PlayBody,
    responses(
        (status = 200, description = "Animation started"),
        (status = 404, description = "Persona, VRM, or animation not found"),
    ),
)]
pub async fn play_vrma(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    path: SpawnedPersonaPath,
    Json(body): Json<PlayBody>,
) -> HttpResult {
    let vrma = vrm_api.vrma(path.entity, body.asset).await?;

    let mut args = PlayVrma {
        vrma,
        transition_duration: Duration::from_secs(0),
        repeat: RepeatAnimation::Never,
        reset_spring_bones: true,
    };
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
    vrma_api
        .play(args, body.wait_for_completion.unwrap_or_default())
        .await
        .into_http_result()
}

/// Stop a VRMA animation on a persona's VRM.
#[utoipa::path(
    post,
    path = "/vrm/vrma/stop",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = AssetRequest,
    responses(
        (status = 200, description = "Animation stopped"),
        (status = 404, description = "Persona, VRM, or animation not found"),
    ),
)]
pub async fn stop_vrma(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    path: SpawnedPersonaPath,
    Json(body): Json<AssetRequest>,
) -> HttpResult {
    let vrma = vrm_api.vrma(path.entity, body.asset).await?;
    vrma_api.stop(vrma).await.into_http_result()
}
