use crate::route::gpt::{GptQuery, to_entity};
use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::prelude::GptApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};

/// Check the model used by the GPT API.
///
/// ### Path
///
/// `GET /gpt/model`
///
/// ### Queries
/// - `vrm`: Optional VRM entity ID to check specific settings for that VRM.
pub async fn get(State(api): State<GptApi>, Query(query): Query<GptQuery>) -> HttpResult<String> {
    api.model(query.vrm_entity()).await.into_http_result()
}

/// Set the model used by the GPT API.
///
/// ### Path
///
/// `PUT /gpt/model`
pub async fn put(
    State(api): State<GptApi>,
    Json(request): Json<PutChatGptModelRequest>,
) -> HttpResult<()> {
    api.save_model(request.model, to_entity(request.vrm))
        .await
        .into_http_result()
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PutChatGptModelRequest {
    /// The model name to use for the GPT API.
    pub model: String,
    /// Optional VRM entity ID to set specific settings for that VRM.
    pub vrm: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::route::gpt::model::PutChatGptModelRequest;
    use crate::tests::{assert_response, call, test_app};
    use axum::http;
    use axum::http::Request;
    use bevy::prelude::*;
    use homunculus_api::prelude::GptApi;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn get_default_global_model() {
        let (mut app, router) = test_app();
        let request = Request::get("/gpt/model")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, GptApi::DEFAULT_MODEL.to_string()).await;
    }

    #[tokio::test]
    async fn get_global_model() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::global::model", &"gpt-4o")
            .unwrap();
        let request = Request::get("/gpt/model")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, "gpt-4o".to_string()).await;
    }

    #[tokio::test]
    async fn get_default_vrm_model() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/model?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, GptApi::DEFAULT_MODEL.to_string()).await;
    }

    #[tokio::test]
    async fn get_vrm_model() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::vrm::Avatar::model", "gpt-4o")
            .unwrap();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/model?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, "gpt-4o".to_string()).await;
    }

    #[tokio::test]
    async fn put_global_model() {
        let (mut app, router) = test_app();
        let request = Request::put("/gpt/model")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutChatGptModelRequest {
                    model: "gpt-4o".to_string(),
                    vrm: None,
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<String>("gpt::global::model")
            .unwrap();
        assert_eq!(model, "gpt-4o");
    }

    #[tokio::test]
    async fn put_vrm_model() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::put("/gpt/model")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutChatGptModelRequest {
                    model: "gpt-4o".to_string(),
                    vrm: Some(vrm.to_bits()),
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<String>("gpt::vrm::Avatar::model")
            .unwrap();
        assert_eq!(model, "gpt-4o");
    }
}
