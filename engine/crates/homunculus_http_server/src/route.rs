use axum::Json;
use homunculus_core::prelude::AssetId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub(crate) mod app;
pub(crate) mod assets;
pub(crate) mod audio;
pub(crate) mod coordinates;
pub(crate) mod dialog;
pub(crate) mod displays;
pub(crate) mod effects;
pub(crate) mod entities;
pub(crate) mod info;
pub(crate) mod mods;
pub(crate) mod persona;
pub(crate) mod preferences;
pub(crate) mod processes;
pub(crate) mod rpc;
pub(crate) mod settings;
pub(crate) mod shadow_panel;
pub(crate) mod signals;
pub(crate) mod stt;
pub(crate) mod webviews;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct AssetRequest {
    pub asset: AssetId,
}

/// Returns a simple health check response.
#[utoipa::path(
    get,
    path = "/health",
    tag = "app",
    responses(
        (status = 200, description = "Health check passed", body = HealthResponse),
    ),
)]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}
