use crate::gpt::{ChatGptModels, GptApi};
use crate::prelude::ApiResult;
use bevy::prelude::*;
use bevy_flurx::action::once;

impl GptApi {
    /// Fetches the available models from the ChatGPT API.
    pub async fn available_models(&self) -> ApiResult<ChatGptModels> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(obtain_models)).await })
            .await
    }
}

fn obtain_models(models: Res<ChatGptModels>) -> ChatGptModels {
    models.clone()
}
