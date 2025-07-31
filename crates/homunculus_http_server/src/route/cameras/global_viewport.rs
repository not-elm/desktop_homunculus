use axum::extract::{Query, State};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{CameraApi, GlobalViewportArgs};
use homunculus_core::prelude::GlobalViewport;

/// Get the global viewport from world coordinates.
///
/// ### Path
///
/// `GET /cameras/global-viewport`
///
/// ### Queries
///
/// - `x`: Optional x-coordinate in world space.
/// - `y`: Optional y-coordinate in world space.
/// - `z`: Optional z-coordinate in world space.
pub async fn global_viewport(
    State(api): State<CameraApi>,
    Query(query): Query<GlobalViewportArgs>,
) -> HttpResult<GlobalViewport> {
    api.global_viewport(query).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::route::cameras::tests::spawn_window_and_camera;
    use crate::tests::{assert_response, test_app};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::*;
    use homunculus_core::prelude::{Coordinate, GlobalViewport};

    #[tokio::test]
    async fn test_convert_to_global_viewport_from_world() {
        let (mut app, router) = test_app();
        spawn_window_and_camera(&mut app);

        let global_viewport: GlobalViewport = app
            .world_mut()
            .run_system_once(|coordinate: Coordinate| coordinate.to_global_by_world(Vec3::ZERO))
            .unwrap()
            .unwrap();
        let request = axum::http::Request::get("/cameras/global-viewport?x=0&y=0&z=0")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, global_viewport).await;
    }
}
