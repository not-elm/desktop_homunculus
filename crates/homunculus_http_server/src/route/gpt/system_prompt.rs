use crate::route::gpt::{GptQuery, to_entity};
use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::prelude::GptApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};

/// Fetch the system prompt.
///
/// ### Path
///
/// `GET /gpt/system-prompt`
///
/// ### Queries
/// - `vrm`: Optional VRM entity ID to check specific settings for that VRM.
pub async fn get(State(api): State<GptApi>, Query(query): Query<GptQuery>) -> HttpResult<String> {
    api.system_prompt(query.vrm_entity())
        .await
        .into_http_result()
}

/// Set the system prompt.
///
/// ### Path
///
/// `PUT /gpt/system-prompt`
pub async fn put(
    State(api): State<GptApi>,
    Json(request): Json<PutSystemPromptRequest>,
) -> HttpResult<()> {
    api.save_system_prompt(request.system_prompt, to_entity(request.vrm))
        .await
        .into_http_result()
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PutSystemPromptRequest {
    /// The model name to use for the GPT API.
    #[serde(rename = "systemPrompt")]
    pub system_prompt: String,
    /// Optional VRM entity ID to set specific settings for that VRM.
    pub vrm: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::route::gpt::system_prompt::PutSystemPromptRequest;
    use crate::tests::{assert_response, call, test_app};
    use axum::http;
    use axum::http::Request;
    use bevy::prelude::*;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn get_global_system_prompt() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::global::system_prompt", &"message")
            .unwrap();
        let request = Request::get("/gpt/system-prompt")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, "message".to_string()).await;
    }

    #[tokio::test]
    async fn put_global_system_prompt() {
        let (mut app, router) = test_app();
        let request = Request::put("/gpt/system-prompt")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutSystemPromptRequest {
                    system_prompt: "message".to_string(),
                    vrm: None,
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<String>("gpt::global::system_prompt")
            .unwrap();
        assert_eq!(model, "message");
    }

    #[tokio::test]
    async fn put_vrm_system_prompt() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::put("/gpt/system-prompt")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutSystemPromptRequest {
                    system_prompt: "message".to_string(),
                    vrm: Some(vrm.to_bits()),
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<String>("gpt::vrm::Avatar::system_prompt")
            .unwrap();
        assert_eq!(model, "message");
    }
}
