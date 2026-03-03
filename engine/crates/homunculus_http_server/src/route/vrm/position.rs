use crate::extract::EntityId;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{PositionResponse, VrmApi};

/// Get the position of a VRM model.
#[utoipa::path(
    get,
    path = "/position",
    tag = "vrm",
    params(("entity" = String, Path, description = "Entity ID")),
    responses(
        (status = 200, description = "VRM position data", body = PositionResponse),
        (status = 404, description = "Entity not found"),
    ),
)]
pub async fn get(
    State(api): State<VrmApi>,
    EntityId(entity): EntityId,
) -> HttpResult<PositionResponse> {
    api.position(entity).await.into_http_result()
}
