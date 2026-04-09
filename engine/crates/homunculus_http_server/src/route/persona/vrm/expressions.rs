use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{ExpressionsResponse, VrmApi};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::route::persona::SpawnedPersonaPath;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WeightsBody {
    pub weights: std::collections::HashMap<String, f32>,
}

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
    path: SpawnedPersonaPath,
) -> HttpResult<ExpressionsResponse> {
    api.list_expressions(path.entity).await.into_http_result()
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
    path: SpawnedPersonaPath,
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
pub async fn clear_expressions(State(api): State<VrmApi>, path: SpawnedPersonaPath) -> HttpResult {
    api.clear_expressions(path.entity).await.into_http_result()
}
