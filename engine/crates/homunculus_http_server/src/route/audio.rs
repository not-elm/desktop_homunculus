use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use homunculus_api::prelude::axum::IntoHttpResult;
use homunculus_api::prelude::{AudioBgmApi, AudioSeApi};
use homunculus_audio::prelude::{BgmStatus, FadeTween};
use homunculus_core::prelude::AssetId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// --- SE ---

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct SePlayRequest {
    pub asset: AssetId,
    #[serde(default = "default_volume")]
    pub volume: f64,
    #[serde(default = "default_speed")]
    pub speed: f64,
    #[serde(default)]
    pub panning: f64,
}

fn default_volume() -> f64 {
    1.0
}

fn default_speed() -> f64 {
    1.0
}

/// Play a one-shot sound effect.
#[utoipa::path(
    post,
    path = "/se",
    tag = "audio",
    request_body = SePlayRequest,
    responses(
        (status = 204, description = "Sound effect played"),
        (status = 400, description = "Invalid request"),
    ),
)]
pub async fn se_play(
    State(api): State<AudioSeApi>,
    Json(body): Json<SePlayRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.play(body.asset, body.volume, body.speed, body.panning)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| e.into_response())
}

// --- BGM ---

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct BgmPlayRequest {
    pub asset: AssetId,
    #[serde(rename = "loop", default = "default_true")]
    pub is_loop: bool,
    #[serde(default = "default_volume")]
    pub volume: f64,
    #[serde(default = "default_speed")]
    pub speed: f64,
    #[serde(rename = "fadeIn")]
    #[schema(value_type = Option<Object>)]
    pub fade_in: Option<FadeTween>,
}

fn default_true() -> bool {
    true
}

/// Play background music (replaces current BGM).
#[utoipa::path(
    post,
    path = "/bgm",
    tag = "audio",
    request_body = BgmPlayRequest,
    responses(
        (status = 204, description = "BGM playback started"),
        (status = 400, description = "Invalid request"),
    ),
)]
pub async fn bgm_play(
    State(api): State<AudioBgmApi>,
    Json(body): Json<BgmPlayRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.play(
        body.asset,
        body.is_loop,
        body.volume,
        body.speed,
        body.fade_in,
    )
    .await
    .map(|_| StatusCode::NO_CONTENT)
    .map_err(|e| e.into_response())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct BgmStopRequest {
    #[serde(rename = "fadeOut")]
    #[schema(value_type = Option<Object>)]
    pub fade_out: Option<FadeTween>,
}

/// Stop background music.
#[utoipa::path(
    post,
    path = "/bgm/stop",
    tag = "audio",
    request_body(content = Option<BgmStopRequest>, description = "Optional fade-out settings"),
    responses(
        (status = 204, description = "BGM stopped"),
        (status = 409, description = "BGM not playing"),
    ),
)]
pub async fn bgm_stop(
    State(api): State<AudioBgmApi>,
    body: Option<Json<BgmStopRequest>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let fade_out = body.and_then(|b| b.0.fade_out);
    api.stop(fade_out)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| e.into_response())
}

/// Pause background music.
#[utoipa::path(
    post,
    path = "/bgm/pause",
    tag = "audio",
    responses(
        (status = 204, description = "BGM paused"),
        (status = 409, description = "BGM not playing"),
    ),
)]
pub async fn bgm_pause(
    State(api): State<AudioBgmApi>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.pause()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| e.into_response())
}

/// Resume background music.
#[utoipa::path(
    post,
    path = "/bgm/resume",
    tag = "audio",
    responses(
        (status = 204, description = "BGM resumed"),
        (status = 409, description = "BGM not paused"),
    ),
)]
pub async fn bgm_resume(
    State(api): State<AudioBgmApi>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.resume()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| e.into_response())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct BgmUpdateRequest {
    pub volume: Option<f64>,
    pub speed: Option<f64>,
    #[schema(value_type = Option<Object>)]
    pub tween: Option<FadeTween>,
}

/// Update BGM settings (volume, speed).
#[utoipa::path(
    patch,
    path = "/bgm",
    tag = "audio",
    request_body = BgmUpdateRequest,
    responses(
        (status = 204, description = "BGM settings updated"),
        (status = 409, description = "BGM not playing"),
    ),
)]
pub async fn bgm_update(
    State(api): State<AudioBgmApi>,
    Json(body): Json<BgmUpdateRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.update(body.volume, body.speed, body.tween)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| e.into_response())
}

/// Get current BGM status.
#[utoipa::path(
    get,
    path = "/bgm",
    tag = "audio",
    responses(
        (status = 200, description = "Current BGM status", body = BgmStatus),
    ),
)]
pub async fn bgm_status(
    State(api): State<AudioBgmApi>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    api.status().await.into_http_result()
}
