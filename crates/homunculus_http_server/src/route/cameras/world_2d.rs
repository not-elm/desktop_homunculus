use axum::extract::*;
use bevy::math::Vec2;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{CameraApi, OptionalGlobalViewport};

/// Convert a global viewport to a 2D world position.
///
/// ### Path
///
/// `GET /cameras/world-2d`
///
/// ### Queries
///
/// - `x`: Optional x-coordinate in global viewport.
/// - `y`: Optional y-coordinate in global viewport.
pub async fn world_2d(
    State(api): State<CameraApi>,
    Query(query): Query<OptionalGlobalViewport>,
) -> HttpResult<Vec2> {
    api.world_2d(query).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::route::cameras::tests::spawn_window_and_camera;
    use crate::tests::{assert_response, test_app};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;
    use homunculus_core::prelude::{Coordinate, GlobalViewport};

    #[tokio::test]
    async fn test_convert_world_2d_pos_from_global_viewport() {
        let (mut app, router) = test_app();
        spawn_window_and_camera(&mut app);

        let world_pos: Vec2 = app
            .world_mut()
            .run_system_once(|coordinate: Coordinate| {
                coordinate.to_world_2d_by_global(GlobalViewport(Vec2::new(0., 0.)))
            })
            .unwrap()
            .unwrap();
        let request = axum::http::Request::get("/cameras/world-2d?x=0&y=0")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, world_pos).await;
    }
}
