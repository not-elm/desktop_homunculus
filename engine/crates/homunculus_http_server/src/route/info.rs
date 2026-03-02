//! `/info` provides application metadata for capability discovery.

use axum::Json;
use axum::extract::State;
use homunculus_api::mods::ModsApi;
use homunculus_api::prelude::axum::HttpResult;
use homunculus_core::prelude::ModInfo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub(crate) const FEATURES: &[&str] = &[
    "vrm", "audio", "webviews", "effects", "speech", "signals", "mods",
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub os: String,
    pub arch: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub platform: PlatformInfo,
    pub features: Vec<String>,
    pub mods: Vec<ModInfo>,
}

/// Returns application metadata including version, platform, features, and loaded mods.
#[utoipa::path(
    get,
    path = "/info",
    tag = "app",
    responses(
        (status = 200, description = "Application info", body = AppInfo),
    ),
)]
pub async fn get(State(mods_api): State<ModsApi>) -> HttpResult<AppInfo> {
    let mod_list = mods_api.list().await?;
    Ok(Json(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: PlatformInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
        features: FEATURES.iter().map(|s| (*s).to_string()).collect(),
        mods: mod_list,
    }))
}
