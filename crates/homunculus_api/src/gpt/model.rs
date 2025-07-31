use crate::error::ApiResult;
use crate::gpt::GptApi;
use bevy::prelude::*;

impl GptApi {
    pub const DEFAULT_MODEL: &'static str = "gpt-4o-mini-search-preview";

    /// Fetches the model used by ChatGPT.
    ///
    /// ### Parameters
    /// - `vrm`:If provided, it will load the VRM-specific system prompt.
    pub async fn model(&self, vrm: Option<Entity>) -> ApiResult<String> {
        self.load_option("model", Self::DEFAULT_MODEL.to_string(), vrm)
            .await
    }

    /// Sets the model used by ChatGPT.
    ///
    /// ### Parameters
    /// - `vrm`:If provided, it will load the VRM-specific system prompt.
    pub async fn save_model(&self, model: String, vrm: Option<Entity>) -> ApiResult {
        self.save_option("model", model, vrm).await
    }
}
