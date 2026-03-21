use crate::extract::character::VrmGuard;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{PositionResponse, VrmApi};

/// Get the position of a VRM model.
#[utoipa::path(
    get,
    path = "/position",
    tag = "vrm",
    params(("id" = String, Path, description = "Character ID")),
    responses(
        (status = 200, description = "VRM position data", body = PositionResponse),
        (status = 404, description = "Character not found"),
        (status = 422, description = "No VRM attached"),
    ),
)]
pub async fn get(
    State(api): State<VrmApi>,
    VrmGuard { entity, .. }: VrmGuard,
) -> HttpResult<PositionResponse> {
    api.position(entity).await.into_http_result()
}
