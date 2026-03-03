use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::ShadowPanelApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Get the current alpha value of the shadow panel.
#[utoipa::path(
    get,
    path = "/alpha",
    tag = "shadow-panel",
    responses(
        (status = 200, description = "Current alpha value", body = f32),
        (status = 400, description = "Shadow panel not available"),
    ),
)]
pub async fn get(State(api): State<ShadowPanelApi>) -> HttpResult<f32> {
    api.alpha().await.into_http_result()
}

/// Set the alpha value of the shadow panel.
///
/// The value set here is saved in the internal database under the key `shadow_panel::alpha`.
/// When the application starts, the alpha value is read from this key and applied to the shadow panel's material.
#[utoipa::path(
    put,
    path = "/alpha",
    tag = "shadow-panel",
    request_body = ShadowPanelPutBody,
    responses(
        (status = 200, description = "Alpha value updated"),
        (status = 400, description = "Shadow panel not available"),
    ),
)]
pub async fn put(
    State(api): State<ShadowPanelApi>,
    Json(body): Json<ShadowPanelPutBody>,
) -> HttpResult {
    api.set_alpha(body.alpha).await.into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, ToSchema)]
pub struct ShadowPanelPutBody {
    /// The alpha value for the shadow panel.
    ///
    /// Range: `0.0` (fully transparent) to `1.0` (fully opaque).
    pub alpha: f32,
}
