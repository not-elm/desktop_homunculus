use crate::gpt::{GptApi, HomunculusChatGpt};
use crate::prelude::ApiResult;
use bevy::prelude::*;

impl GptApi {
    /// Fetches the system prompt used by ChatGPT.
    ///
    /// ### Parameters
    ///
    /// - `vrm`:If provided, it will load the VRM-specific system prompt.
    pub async fn system_prompt(&self, vrm: Option<Entity>) -> ApiResult<String> {
        self.load_option(
            "system_prompt",
            HomunculusChatGpt::ELMER_DEFAULT_SYSTEM_PROMPT.to_string(),
            vrm,
        )
        .await
    }

    /// Sets the system prompt used by ChatGPT.
    ///
    /// ### Parameters
    ///
    /// - `vrm`: If provided, it will load the VRM-specific system prompt.
    pub async fn save_system_prompt(
        &self,
        system_prompt: String,
        vrm: Option<Entity>,
    ) -> ApiResult {
        self.save_option("system_prompt", system_prompt, vrm).await
    }
}
