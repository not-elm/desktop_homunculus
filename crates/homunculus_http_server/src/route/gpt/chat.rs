use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{ChatGptResponse, ChatVrmOptions, GptApi, SpeakVoiceVoxOptions};
use homunculus_effects::Entity;
use serde::{Deserialize, Serialize};

/// Send a chat message to the GPT API.
///
/// By providing the appropriate values in the request body, you can make a specific VRM speak.
///
/// ### Path
///
/// `POST /gpt/chat`
pub async fn chat(
    State(api): State<GptApi>,
    Json(request): Json<ChatRequest>,
) -> HttpResult<ChatGptResponse> {
    api.chat(
        request.user_message,
        request.options.map(|o| ChatVrmOptions {
            vrm: Entity::from_bits(o.vrm),
            options: o.options,
        }),
    )
    .await
    .into_http_result()
}

/// Request body for the `POST /gpt/chat` route.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ChatRequest {
    /// The user message to send to the GPT API.
    #[serde(rename = "userMessage")]
    user_message: String,

    /// If you want to have a conversation with a specific VRM, pass the VRM entity from this option.
    ///
    /// You can also use a text-to-speech engine to make it speak.
    options: Option<ChatVrmRequestOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatVrmRequestOptions {
    /// The speaker's VRM entity.
    pub vrm: u64,

    /// If you want to have a conversation with a specific VRM, pass the VRM entity from this option.
    ///
    /// You can also use a text-to-speech engine to make it speak.
    #[serde(flatten)]
    pub options: SpeakVoiceVoxOptions,
}
