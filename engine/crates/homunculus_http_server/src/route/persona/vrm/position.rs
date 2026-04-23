use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::vrm::{PositionResponse, VrmApi};

use crate::route::persona::SpawnedPersonaPath;

/// Get the position of a persona's VRM.
#[utoipa::path(
    get,
    path = "/vrm/position",
    tag = "personas",
    params(("id" = String, Path, description = "Persona ID")),
    responses(
        (status = 200, description = "VRM position data", body = PositionResponse),
        (status = 404, description = "Persona or VRM not found"),
    ),
)]
pub async fn get_position(
    State(api): State<VrmApi>,
    path: SpawnedPersonaPath,
) -> HttpResult<PositionResponse> {
    api.position(path.entity).await.into_http_result()
}
