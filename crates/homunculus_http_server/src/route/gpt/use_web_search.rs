use crate::route::gpt::{GptQuery, to_entity};
use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::prelude::GptApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};

/// Check if web search is enabled for the GPT API.
///
/// ### Path
///
/// `GET /gpt/use-web-search`
///
/// ### Notes
///
/// Notes that web search is only supported for specific models.
/// Most likely, only models with `search` in their name support it.
///
/// ### Queries
///
/// - `vrm`: Optional VRM entity ID to check specific settings for that VRM.
pub async fn get(State(api): State<GptApi>, Query(query): Query<GptQuery>) -> HttpResult<bool> {
    api.use_web_search(query.vrm_entity())
        .await
        .into_http_result()
}

/// Set whether web search is enabled for the GPT API.
///
/// ### Path
///
/// `PUT /gpt/use-web-search`
///
/// ### Notes
///
/// Notes that web search is only supported for specific models.
/// Most likely, only models with `search` in their name support it.
pub async fn put(
    State(api): State<GptApi>,
    Json(request): Json<PutUseWebSearchRequest>,
) -> HttpResult<()> {
    api.save_use_web_search(request.use_web_search, to_entity(request.vrm))
        .await
        .into_http_result()
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PutUseWebSearchRequest {
    /// Whether to use web search.
    #[serde(rename = "useWebSearch")]
    pub use_web_search: bool,
    /// Optional VRM entity ID to set specific settings for that VRM.
    pub vrm: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::route::gpt::use_web_search::PutUseWebSearchRequest;
    use crate::tests::{assert_response, call, test_app};
    use axum::http;
    use axum::http::Request;
    use bevy::prelude::*;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn get_default_global_use_web_search() {
        let (mut app, router) = test_app();
        let request = Request::get("/gpt/use-web-search")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, true).await;
    }

    #[tokio::test]
    async fn get_global_use_web_search() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::global::use_web_search", &false)
            .unwrap();
        let request = Request::get("/gpt/use-web-search")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, false).await;
    }

    #[tokio::test]
    async fn get_default_vrm_use_web_search() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/use-web-search?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, true).await;
    }

    #[tokio::test]
    async fn get_vrm_use_web_search() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::vrm::Avatar::use_web_search", &false)
            .unwrap();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/use-web-search?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, false).await;
    }

    #[tokio::test]
    async fn put_global_use_web_search() {
        let (mut app, router) = test_app();
        let request = Request::put("/gpt/use-web-search")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutUseWebSearchRequest {
                    use_web_search: true,
                    vrm: None,
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let use_web_search = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<bool>("gpt::global::use_web_search")
            .unwrap();
        assert!(use_web_search);
    }

    #[tokio::test]
    async fn put_vrm_use_web_search() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::put("/gpt/use-web-search")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutUseWebSearchRequest {
                    use_web_search: true,
                    vrm: Some(vrm.to_bits()),
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let use_web_search = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<bool>("gpt::vrm::Avatar::use_web_search")
            .unwrap();
        assert!(use_web_search);
    }
}
