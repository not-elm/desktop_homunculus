use crate::extract::character::VrmGuard;
use axum::Json;
use axum::extract::State;
use base64::Engine;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{ApiError, SpeakTimelineOptions, SpeechApi, TimelineKeyframe};
use serde::Deserialize;
use utoipa::ToSchema;

/// Speak with a timeline of expression keyframes and audio data.
#[utoipa::path(
    post,
    path = "/speech/timeline",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = TimelineBody,
    responses(
        (status = 200, description = "Speech timeline started"),
        (status = 400, description = "Invalid audio data"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn timeline(
    State(api): State<SpeechApi>,
    VrmGuard { entity, .. }: VrmGuard,
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

    api.speak_with_timeline(entity, wav, body.keyframes, body.options.unwrap_or_default())
        .await
        .into_http_result()
}

#[derive(Deserialize, ToSchema)]
pub struct TimelineBody {
    pub audio: String,
    pub keyframes: Vec<TimelineKeyframe>,
    #[serde(flatten)]
    #[schema(value_type = Option<Object>)]
    pub options: Option<SpeakTimelineOptions>,
}
