use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::SettingsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Get the current frame rate (FPS).
#[utoipa::path(
    get,
    path = "/fps",
    tag = "settings",
    responses(
        (status = 200, description = "Current FPS", body = f64),
    ),
)]
pub async fn get_fps(State(api): State<SettingsApi>) -> HttpResult<f64> {
    Ok(api.fps().await).into_http_result()
}

/// Set the frame rate (FPS). Persists and applies immediately.
#[utoipa::path(
    put,
    path = "/fps",
    tag = "settings",
    request_body = SetFpsBody,
    responses(
        (status = 200, description = "FPS updated"),
    ),
)]
pub async fn set_fps(State(api): State<SettingsApi>, Json(body): Json<SetFpsBody>) -> HttpResult {
    api.set_fps(body.fps).await.into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetFpsBody {
    /// Frame rate in frames per second.
    pub fps: f64,
}
