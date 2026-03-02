use axum::extract::*;
use bevy::math::Vec2;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{CameraApi, OptionalGlobalViewport};

/// Convert a global viewport position to a 2D world position.
#[utoipa::path(
    get,
    path = "/to-world",
    tag = "coordinates",
    params(
        ("x" = Option<f32>, Query, description = "X-coordinate in global viewport"),
        ("y" = Option<f32>, Query, description = "Y-coordinate in global viewport"),
    ),
    responses(
        (status = 200, description = "2D world position", body = [f32; 2]),
        (status = 500, description = "Conversion failed"),
    ),
)]
pub async fn world_2d(
    State(api): State<CameraApi>,
    Query(query): Query<OptionalGlobalViewport>,
) -> HttpResult<Vec2> {
    api.world_2d(query).await.into_http_result()
}
