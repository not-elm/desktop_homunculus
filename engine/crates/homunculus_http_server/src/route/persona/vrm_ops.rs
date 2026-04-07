use crate::route::AssetRequest;
use axum::Json;
use axum::extract::{Path, State};
use base64::Engine;
use bevy::prelude::Entity;
use bevy_vrm1::vrm::VrmBone;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{
    ApiError, SpeakTimelineOptions, SpeechApi, VrmAnimationApi, VrmaInfo,
};
use homunculus_api::vrm::{
    ExpressionsResponse, PositionResponse, SpringBoneChainsResponse, SpringBonePropsUpdate, VrmApi,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::PersonaPath;

// ---------------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------------

/// List all expressions and their current weights for a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/expressions",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Expression weights", body = ExpressionsResponse),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn list_expressions(
    State(api): State<VrmApi>,
    path: PersonaPath,
) -> HttpResult<ExpressionsResponse> {
    api.list_expressions(path.entity).await.into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WeightsBody {
    pub weights: std::collections::HashMap<String, f32>,
}

/// Modify specific expression weights (merges with current weights).
#[utoipa::path(
    patch,
    path = "/vrm/expressions",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = WeightsBody,
    responses(
        (status = 200, description = "Expressions modified"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn modify_expressions(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Json(body): Json<WeightsBody>,
) -> HttpResult {
    api.modify_expressions(path.entity, body.weights)
        .await
        .into_http_result()
}

/// Clear all expression weights.
#[utoipa::path(
    delete,
    path = "/vrm/expressions",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Expressions cleared"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn clear_expressions(
    State(api): State<VrmApi>,
    path: PersonaPath,
) -> HttpResult {
    api.clear_expressions(path.entity).await.into_http_result()
}

// ---------------------------------------------------------------------------
// VRMA
// ---------------------------------------------------------------------------

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
    path: PersonaPath,
) -> HttpResult<Vec<VrmaInfo>> {
    api.list_all(path.entity).await.into_http_result()
}

/// Play a VRMA animation on a persona's VRM.
#[utoipa::path(
    post,
    path = "/vrm/vrma/play",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = crate::route::vrm::vrma::PlayBody,
    responses(
        (status = 200, description = "Animation started"),
        (status = 404, description = "Persona, VRM, or animation not found"),
    ),
)]
pub async fn play_vrma(
    State(vrm_api): State<VrmApi>,
    State(vrma_api): State<VrmAnimationApi>,
    path: PersonaPath,
    Json(body): Json<crate::route::vrm::vrma::PlayBody>,
) -> HttpResult {
    use bevy::animation::RepeatAnimation;
    use bevy_vrm1::prelude::PlayVrma;
    use std::time::Duration;

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
            crate::route::vrm::vrma::Repeat::Forever => RepeatAnimation::Forever,
            crate::route::vrm::vrma::Repeat::Never => RepeatAnimation::Never,
            crate::route::vrm::vrma::Repeat::Count { count } => RepeatAnimation::Count(count),
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
    path: PersonaPath,
    Json(body): Json<AssetRequest>,
) -> HttpResult {
    let vrma = vrm_api.vrma(path.entity, body.asset).await?;
    vrma_api.stop(vrma).await.into_http_result()
}

// ---------------------------------------------------------------------------
// Position
// ---------------------------------------------------------------------------

/// Get the position of a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/position",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "VRM position data", body = PositionResponse),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn get_position(
    State(api): State<VrmApi>,
    path: PersonaPath,
) -> HttpResult<PositionResponse> {
    api.position(path.entity).await.into_http_result()
}

// ---------------------------------------------------------------------------
// Bone
// ---------------------------------------------------------------------------

/// Get the entity ID of a specific bone in a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/bone/{bone_name}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("bone_name" = String, Path, description = "Bone name"),
    ),
    responses(
        (status = 200, description = "Bone entity ID", body = String),
        (status = 404, description = "Persona, VRM, or bone not found"),
    ),
)]
pub async fn get_bone(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Path((_, bone_name)): Path<(String, String)>,
) -> HttpResult<Entity> {
    let bone_name = VrmBone(bone_name);
    api.bone(path.entity, bone_name).await.into_http_result()
}

// ---------------------------------------------------------------------------
// Look
// ---------------------------------------------------------------------------

/// Set look-at to follow the cursor.
#[utoipa::path(
    put,
    path = "/vrm/look/cursor",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Look-at cursor mode set"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn look_cursor(State(api): State<VrmApi>, path: PersonaPath) -> HttpResult {
    api.look_at_cursor(path.entity).await.into_http_result()
}

/// Set look-at target to another entity.
#[utoipa::path(
    put,
    path = "/vrm/look/target/{target}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("target" = String, Path, description = "Target entity ID"),
    ),
    responses(
        (status = 200, description = "Look-at target set"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn look_target(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Path((_, target)): Path<(String, Entity)>,
) -> HttpResult {
    api.look_at_target(path.entity, target)
        .await
        .into_http_result()
}

/// Disable look-at control.
#[utoipa::path(
    delete,
    path = "/vrm/look",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Look-at disabled"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn unlook(State(api): State<VrmApi>, path: PersonaPath) -> HttpResult {
    api.unlook(path.entity).await.into_http_result()
}

// ---------------------------------------------------------------------------
// Spring bones
// ---------------------------------------------------------------------------

/// List all spring bone chains for a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/spring-bones",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "Spring bone chains", body = SpringBoneChainsResponse),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn list_spring_bones(
    State(api): State<VrmApi>,
    path: PersonaPath,
) -> HttpResult<SpringBoneChainsResponse> {
    api.list_spring_bones(path.entity)
        .await
        .into_http_result()
}

/// Update properties of a spring bone chain.
#[utoipa::path(
    patch,
    path = "/vrm/spring-bones/{chain_id}",
    tag = "personas",
    params(
        ("id" = String, Path, description = "Persona ID"),
        ("chain_id" = String, Path, description = "Spring bone chain entity ID"),
    ),
    request_body = SpringBonePropsUpdate,
    responses(
        (status = 200, description = "Spring bone properties updated"),
        (status = 404, description = "Persona, VRM, or chain not found"),
    ),
)]
pub async fn patch_spring_bones(
    State(api): State<VrmApi>,
    path: PersonaPath,
    Path((_, chain_id)): Path<(String, Entity)>,
    Json(body): Json<SpringBonePropsUpdate>,
) -> HttpResult {
    api.set_spring_bone_props(path.entity, chain_id, body)
        .await
        .into_http_result()
}

// ---------------------------------------------------------------------------
// Speech timeline
// ---------------------------------------------------------------------------

#[derive(Deserialize, ToSchema)]
pub struct TimelineBody {
    pub audio: String,
    pub keyframes: Vec<homunculus_api::prelude::TimelineKeyframe>,
    #[serde(flatten)]
    #[schema(value_type = Option<Object>)]
    pub options: Option<SpeakTimelineOptions>,
}

/// Speak with a timeline of expression keyframes and audio data.
#[utoipa::path(
    post,
    path = "/vrm/speech/timeline",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    request_body = TimelineBody,
    responses(
        (status = 200, description = "Speech timeline started"),
        (status = 400, description = "Invalid audio data"),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn speech_timeline(
    State(api): State<SpeechApi>,
    path: PersonaPath,
    Json(body): Json<TimelineBody>,
) -> HttpResult {
    const MAX_AUDIO_BYTES: usize = 5 * 1024 * 1024;

    let wav = base64::engine::general_purpose::STANDARD
        .decode(&body.audio)
        .map_err(|e| ApiError::InvalidInput(format!("Invalid base64 audio data: {e}")))?;

    if wav.len() > MAX_AUDIO_BYTES {
        return Err(ApiError::InvalidInput(format!(
            "Decoded audio exceeds {} byte limit",
            MAX_AUDIO_BYTES
        )));
    }

    api.speak_with_timeline(
        path.entity,
        wav,
        body.keyframes,
        body.options.unwrap_or_default(),
    )
    .await
    .into_http_result()
}
