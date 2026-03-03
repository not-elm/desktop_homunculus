use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::EffectsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::AssetId;
use homunculus_effects::prelude::StampOptions;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Show a stamp effect.
#[utoipa::path(
    post,
    path = "/stamps",
    tag = "effects",
    request_body = StampRequestBody,
    responses(
        (status = 200, description = "Stamp effect displayed"),
        (status = 400, description = "Invalid request"),
    ),
)]
pub async fn stamp(
    State(api): State<EffectsApi>,
    Json(body): Json<StampRequestBody>,
) -> HttpResult {
    api.stamp(body.asset, body.options).await.into_http_result()
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct StampRequestBody {
    pub asset: AssetId,
    #[serde(flatten)]
    #[schema(value_type = Option<Object>)]
    pub options: Option<StampOptions>,
}
