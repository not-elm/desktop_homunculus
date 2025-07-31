//! `/preferences` provides methods for managing user preferences.

use axum::Json;
use axum::extract::{Path, State};
use homunculus_api::preferences::PrefsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Load a preference value by key.
///
/// ### Path
///
/// `GET /preferences/:key`
pub async fn load(
    State(api): State<PrefsApi>,
    Path(key): Path<String>,
) -> HttpResult<serde_json::Value> {
    api.load(key).await.into_http_result()
}

/// Save a preference value by key.
///
/// ### Path
///
/// `PUT /preferences/:key`
pub async fn save(
    State(api): State<PrefsApi>,
    Path(key): Path<String>,
    Json(value): Json<serde_json::Value>,
) -> HttpResult {
    api.save(key.clone(), value).await.into_http_result()
}

#[cfg(test)]
mod tests {
    use crate::tests::{assert_response, call, test_app};
    use axum::http::Request;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn test_load_preferences() {
        let (mut app, router) = test_app();
        let value = serde_json::json!({"key": "value"});
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("test", &value)
            .unwrap();
        let request = Request::get("/preferences/test")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, value).await;
    }

    #[tokio::test]
    async fn test_save_preferences() {
        let (mut app, router) = test_app();
        let value = serde_json::json!({"key": "value"});
        let request = Request::put("/preferences/test")
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(value.to_string()))
            .unwrap();
        call(&mut app, router, request).await;
        let loaded_value = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load("test")
            .unwrap();
        assert_eq!(loaded_value, value);
    }
}
