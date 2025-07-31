use crate::error::{ApiError, ApiResult};
use crate::gpt::{GptApi, HomunculusChatGpt, obtain_gpt};
use crate::prelude::{ChatGptResponse, SpeakVoiceVoxOptions, SpeechApi};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
};
use bevy_flurx::prelude::*;
use homunculus_core::prelude::OutputLog;
use homunculus_effects::{Entity, Update};
use serde::{Deserialize, Serialize};
use tracing::info;

impl GptApi {
    /// Sends a chat message to the ChatGPT API.
    ///
    /// If you specify the VRM entity in the options, it will reflect the settings of that VRM.
    /// Additionally, by specifying the Speak options, you can make the VRM speak the message.
    pub async fn chat(
        &self,
        user_message: impl Into<String>,
        options: Option<ChatVrmOptions>,
    ) -> ApiResult<ChatGptResponse> {
        let user_message = user_message.into();
        let speak = SpeechApi::from(self.0.clone());
        let vrm = options.as_ref().map(|o| o.vrm);
        let system_prompt = self.system_prompt(vrm).await?;
        let use_web_search = self.use_web_search(vrm).await?;
        let speaker = self.voicevox_speaker(vrm).await?;
        let model = self.model(vrm).await?;
        let response = self
            .0
            .schedule(move |task| async move {
                let response = task
                    .will(
                        Update,
                        once::run(obtain_gpt).pipe(side_effect::tokio::spawn(
                            move |gpt: HomunculusChatGpt| async move {
                                let messages = vec![
                                    ChatCompletionRequestMessage::System(
                                        ChatCompletionRequestSystemMessageArgs::default()
                                            .content(system_prompt)
                                            .build()?,
                                    ),
                                    ChatCompletionRequestMessage::User(
                                        ChatCompletionRequestUserMessageArgs::default()
                                            .content(user_message)
                                            .build()?,
                                    ),
                                ];
                                gpt.chat(messages, &model, use_web_search).await
                            },
                        )),
                    )
                    .await?;
                Result::<ChatGptResponse, ApiError>::Ok(response)
            })
            .await??;
        if let Some(options) = options {
            start_speak(speak, speaker, response.dialogue.clone(), options);
        }
        Ok(response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatVrmOptions {
    /// The speaker's VRM entity.
    pub vrm: Entity,
    #[serde(flatten)]
    pub options: SpeakVoiceVoxOptions,
}

fn start_speak(api: SpeechApi, speaker: u32, dialogue: String, mut speak: ChatVrmOptions) {
    speak.options.speaker.replace(speaker);
    tokio::task::spawn(async move {
        let vrm = speak.vrm;
        info!("Starting to speak: {dialogue}");
        api.speak_on_voicevox(vrm, vec![dialogue], speak.options)
            .await
            .output_log_if_error("ChatApi::speak_on_voicevox");
    });
}
