use crate::error::ApiResult;
use crate::gpt::GptApi;
use bevy::prelude::*;

impl GptApi {
    /// Get whether ChatGPT uses web search.
    ///
    /// ### Notes
    ///
    /// Notes that web search is only supported for specific models.
    /// Most likely, only models with `search` in their name support it.
    ///
    /// ### Parameters
    ///
    /// - `vrm`:If provided, it will load the VRM-specific system prompt.
    pub async fn use_web_search(&self, vrm: Option<Entity>) -> ApiResult<bool> {
        self.load_option("use_web_search", true, vrm).await
    }

    /// Set whether ChatGPT uses web search.
    ///
    /// ### Notes
    ///
    /// Notes that web search is only supported for specific models.
    /// Most likely, only models with `search` in their name support it.
    ///
    /// ### Parameters
    ///
    /// - `vrm`:If provided, it will load the VRM-specific system prompt.
    pub async fn save_use_web_search(
        &self,
        use_web_search: bool,
        vrm: Option<Entity>,
    ) -> ApiResult {
        self.save_option("use_web_search", use_web_search, vrm)
            .await
    }
}
