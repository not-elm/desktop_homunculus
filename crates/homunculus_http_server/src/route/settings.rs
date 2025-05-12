//! `/settings` provides methods for managing application settings.

use axum::extract::State;
use homunculus_api::prelude::SettingsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Get the current FPS limit.
///
/// ### Path
///
/// `GET /settings/fps`
pub async fn fps(State(api): State<SettingsApi>) -> HttpResult<f64> {
    let fps = api.fps().await;
    Ok(fps.into())
}

/// Set the FPS limit.
///
/// ### Path
///
/// `PUT /settings/fps`
pub async fn set_fps(
    State(api): State<SettingsApi>,
    axum::Json(body): axum::Json<f64>,
) -> HttpResult<()> {
    api.set_fps(body).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, test_app};
    use axum::http::Request;
    use homunculus_api::prelude::SettingsApi;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn test_load_fps() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save(SettingsApi::MAX_FPS, &30.0)
            .unwrap();
        let request = Request::get("/settings/fps")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, 30.0).await;
    }

    #[tokio::test]
    async fn test_save_fps() {
        let (mut app, router) = test_app();
        let request = Request::put("/settings/fps")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(serde_json::json!(30.0).to_string()))
            .unwrap();
        call(&mut app, router, request).await;
        let fps = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load(SettingsApi::MAX_FPS)
            .unwrap();
        assert_eq!(fps, 30.0);
    }
}
