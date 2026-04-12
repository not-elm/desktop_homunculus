//! `/preferences` provides methods for managing user preferences.

use axum::Json;
use axum::extract::{Path, State};
use homunculus_api::preferences::PrefsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// List all saved preference keys.
#[utoipa::path(
    get,
    path = "/",
    tag = "preferences",
    responses(
        (status = 200, description = "List of preference keys", body = Vec<String>),
    ),
)]
pub async fn list_preferences(State(api): State<PrefsApi>) -> HttpResult<Vec<String>> {
    api.list().await.into_http_result()
}

/// Load a preference value by key.
#[utoipa::path(
    get,
    path = "/{key}",
    tag = "preferences",
    params(
        ("key" = String, Path, description = "Preference key"),
    ),
    responses(
        (status = 200, description = "Preference value", body = Object),
        (status = 404, description = "Preference not found"),
    ),
)]
pub async fn load(
    State(api): State<PrefsApi>,
    Path(key): Path<String>,
) -> HttpResult<serde_json::Value> {
    api.load(key).await.into_http_result()
}

/// Save a preference value by key.
#[utoipa::path(
    put,
    path = "/{key}",
    tag = "preferences",
    params(
        ("key" = String, Path, description = "Preference key"),
    ),
    request_body = Object,
    responses(
        (status = 200, description = "Preference saved"),
    ),
)]
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
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_load_preferences() {
        let (mut app, router) = test_app();
        let value = serde_json::json!({"key": "value"});
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save_json("test", &value)
            .unwrap();
        let request = Request::get("/preferences/test")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, value).await;
    }

    #[tokio::test]
    async fn test_list_preferences_empty() {
        let (mut app, router) = test_app();
        let request = Request::get("/preferences")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response::<Vec<String>>(&mut app, router, request, vec![]).await;
    }

    #[tokio::test]
    async fn test_list_preferences_with_entries() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save_json("test-key", &serde_json::json!("value"))
            .unwrap();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save_json("another-key", &serde_json::json!(42))
            .unwrap();
        let request = Request::get("/preferences")
            .body(axum::body::Body::empty())
            .unwrap();
        let response = call(&mut app, router, request).await;
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let mut keys: Vec<String> = serde_json::from_slice(&body).unwrap();
        keys.sort();
        assert_eq!(
            keys,
            vec!["another-key".to_string(), "test-key".to_string()]
        );
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
            .load_json("test")
            .unwrap()
            .unwrap();
        assert_eq!(loaded_value, value);
    }
}
