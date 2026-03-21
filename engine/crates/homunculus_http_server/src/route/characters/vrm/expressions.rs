use crate::extract::character::VrmGuard;
use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{ExpressionsResponse, VrmApi};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// List all expressions and their current weights for a VRM model.
#[utoipa::path(
    get,
    path = "/expressions",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Expression weights", body = ExpressionsResponse),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn list(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
) -> HttpResult<ExpressionsResponse> {
    api.list_expressions(entity).await.into_http_result()
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WeightsBody {
    pub weights: HashMap<String, f32>,
}

/// Set all expression weights for a VRM model (replaces all current weights).
#[utoipa::path(
    put,
    path = "/expressions",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = WeightsBody,
    responses(
        (status = 200, description = "Expressions set"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn set(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<WeightsBody>,
) -> HttpResult {
    api.set_expressions(entity, body.weights)
        .await
        .into_http_result()
}

/// Modify specific expression weights for a VRM model (merges with current weights).
#[utoipa::path(
    patch,
    path = "/expressions",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = WeightsBody,
    responses(
        (status = 200, description = "Expressions modified"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn modify(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<WeightsBody>,
) -> HttpResult {
    api.modify_expressions(entity, body.weights)
        .await
        .into_http_result()
}

/// Clear all expression weights for a VRM model.
#[utoipa::path(
    delete,
    path = "/expressions",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "Expressions cleared"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn clear(State(api): State<VrmApi>, VrmGuard { entity, .. }: VrmGuard) -> HttpResult {
    api.clear_expressions(entity).await.into_http_result()
}

/// Modify mouth expression weights for a VRM model.
#[utoipa::path(
    patch,
    path = "/expressions/mouth",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    request_body = WeightsBody,
    responses(
        (status = 200, description = "Mouth expressions modified"),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn modify_mouth(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
    Json(body): Json<WeightsBody>,
) -> HttpResult {
    api.modify_mouth(entity, body.weights)
        .await
        .into_http_result()
}
