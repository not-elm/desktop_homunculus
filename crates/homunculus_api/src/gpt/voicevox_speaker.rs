use crate::error::ApiResult;
use crate::gpt::GptApi;
use bevy::prelude::*;

impl GptApi {
    pub const DEFAULT_VOICEVOX_SPEAKER: u32 = 48;

    /// Returns the ID of the speaker used to read out the GPT messages.
    pub async fn voicevox_speaker(&self, vrm_entity: Option<Entity>) -> ApiResult<u32> {
        self.load_option(
            "speaker::voicevox",
            Self::DEFAULT_VOICEVOX_SPEAKER,
            vrm_entity,
        )
        .await
    }

    /// Saves the ID of the speaker used to read out the GPT messages.
    pub async fn save_voicevox_speaker(
        &self,
        speaker_id: u32,
        vrm_entity: Option<Entity>,
    ) -> ApiResult {
        self.save_option("speaker::voicevox", speaker_id, vrm_entity)
            .await
    }
}
