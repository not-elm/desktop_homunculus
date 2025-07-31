//! `/effects` provides methods for visual and audio effects.

use crate::route::ModuleSourceRequest;
use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::EffectsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::ModModuleSource;
use homunculus_effects::prelude::StampOptions;
use serde::{Deserialize, Serialize};

/// Play a sound effect.
///
/// ### Path
///
/// `POST /effects/sound`
pub async fn sound(
    State(api): State<EffectsApi>,
    Json(body): Json<ModuleSourceRequest>,
) -> HttpResult {
    api.sound(body.source).await.into_http_result()
}

/// Show a stamp effect.
///
/// ### Path
///
/// `POST /effects/stamp`
pub async fn stamp(
    State(api): State<EffectsApi>,
    Json(body): Json<StampRequestBody>,
) -> HttpResult {
    api.stamp(body.source, body.options)
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StampRequestBody {
    pub source: ModModuleSource,
    #[serde(flatten)]
    pub options: Option<StampOptions>,
}
