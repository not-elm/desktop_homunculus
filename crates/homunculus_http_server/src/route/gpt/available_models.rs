use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{ChatGptModels, GptApi};

/// Fetch the available models from the ChatGPT API.
///
/// ### Path
///
/// `GET /gpt/available-models`
pub async fn available_models(State(api): State<GptApi>) -> HttpResult<ChatGptModels> {
    api.available_models().await.into_http_result()
}
