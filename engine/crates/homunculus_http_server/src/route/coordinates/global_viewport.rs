use axum::extract::{Query, State};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{CameraApi, GlobalViewportArgs};
use homunculus_core::prelude::GlobalViewport;

/// Convert world coordinates to global viewport coordinates.
#[utoipa::path(
    get,
    path = "/to-viewport",
    tag = "coordinates",
    params(
        ("x" = Option<f32>, Query, description = "X-coordinate in world space"),
        ("y" = Option<f32>, Query, description = "Y-coordinate in world space"),
        ("z" = Option<f32>, Query, description = "Z-coordinate in world space"),
    ),
    responses(
        (status = 200, description = "Global viewport coordinates", body = GlobalViewport),
        (status = 500, description = "Conversion failed"),
    ),
)]
pub async fn global_viewport(
    State(api): State<CameraApi>,
    Query(query): Query<GlobalViewportArgs>,
) -> HttpResult<GlobalViewport> {
    api.global_viewport(query).await.into_http_result()
}
