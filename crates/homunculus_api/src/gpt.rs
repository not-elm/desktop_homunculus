//! This module provides an API for interacting with the GPT model, including chat functionality and model management.
//!
//! ## Notes
//!
//! You need to set the environment variable `OPENAI_API_KEY` beforehand.
//! `desktop_homunculus` loads the environment variables from `assets/.env` at application startup, so you can either write it in that file or set it using tools like `launchctl`.

mod all_models;
mod chat;
mod model;
mod system_prompt;
mod use_web_search;
mod voicevox_speaker;

use crate::api;
use crate::error::{ApiError, ApiResult};
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema, WebSearchOptions,
};
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_prefs::PrefsDatabase;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use chat::ChatVrmOptions;

api!(GptApi);

fn obtain_vrm_name(In(vrm): In<Entity>, vrms: Query<&Name>) -> ApiResult<Name> {
    vrms.get(vrm).cloned().map_err(|_| ApiError::EntityNotfound)
}

fn obtain_gpt(gpt: Res<HomunculusChatGpt>) -> HomunculusChatGpt {
    gpt.clone()
}

pub struct GptApiPlugin;

impl Plugin for GptApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fetch_available_models);
    }
}

#[repr(transparent)]
#[derive(Resource, Clone, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct ChatGptModels(pub Vec<String>);

#[derive(Resource, Clone, Deref, DerefMut)]
pub struct HomunculusChatGpt(Client<OpenAIConfig>);

impl HomunculusChatGpt {
    pub const ELMER_DEFAULT_SYSTEM_PROMPT: &'static str =
        include_str!("./gpt/elmer_default_system_prompt.md");
    pub const SYSTEM_PROMPT_RESPONSE_FORMAT: &'static str =
        include_str!("./gpt/system_prompt_response_format.md");

    pub async fn chat(
        &self,
        mut prompts: Vec<ChatCompletionRequestMessage>,
        model: &str,
        use_web_search: bool,
    ) -> ApiResult<ChatGptResponse> {
        prompts.insert(
            0,
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage::from(
                Self::SYSTEM_PROMPT_RESPONSE_FORMAT,
            )),
        );
        let mut args = CreateChatCompletionRequestArgs::default();
        if use_web_search {
            args.web_search_options(WebSearchOptions::default());
        }
        let request = args
            .model(model)
            .messages(prompts)
            .response_format(ResponseFormat::JsonSchema {
                json_schema: ResponseFormatJsonSchema {
                    schema: Some(Self::response_schema()),
                    strict: Some(true),
                    name: "ChatResponse".to_string(),
                    description: None,
                },
            })
            .build()?;
        let response = self.0.chat().create(request).await?;
        let message = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or(ApiError::MissingOpenAIResponseMessage)?;
        let message = message.replace("\n", "\\n");
        serde_json::from_str(&message).map_err(|e| ApiError::InvalidOpenAIResponse(e.to_string()))
    }

    fn response_schema() -> serde_json::Value {
        serde_json::json!({
          "type": "object",
          "properties": {
            "message": {
              "type": "string",
            },
            "dialogue":{
                "type": "string",
            },
            "emotion": {
                "type": "string",
                "enum": [
                    "happy",
                    "sad",
                    "angry",
                    "surprised",
                    "neutral"
                ]
            }
          },
          "required": ["message", "dialogue", "emotion"],
          "additionalProperties": false
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ChatGptResponse {
    pub message: String,
    pub dialogue: String,
    pub emotion: String,
}

fn fetch_available_models(mut commands: Commands) {
    let gpt = HomunculusChatGpt(Client::default());
    let client = gpt.clone();
    let models = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            if let Ok(models) = client.models().list().await {
                models
                    .data
                    .iter()
                    .map(|d| d.id.to_string())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        });
    commands.insert_resource(ChatGptModels(models));
    commands.insert_resource(gpt);
}

impl GptApi {
    async fn load_option<V>(
        &self,
        key: &'static str,
        default_value: V,
        vrm_entity: Option<Entity>,
    ) -> ApiResult<V>
    where
        V: DeserializeOwned + Send + Sync + 'static,
    {
        let vrm_name = self.fetch_vrm_name(vrm_entity).await?;
        self.0
            .schedule(move |task| async move {
                let value = task
                    .will(
                        Update,
                        once::run(load_option).with((vrm_name, key, default_value)),
                    )
                    .await;
                Ok(value)
            })
            .await?
    }

    async fn save_option<V>(
        &self,
        key: &'static str,
        value: V,
        vrm_entity: Option<Entity>,
    ) -> ApiResult
    where
        V: Serialize + Send + Sync + 'static,
    {
        let vrm_name = self.fetch_vrm_name(vrm_entity).await?;
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(save_option).with((vrm_name, key, value)))
                    .await?;
                Ok(())
            })
            .await?
    }

    async fn fetch_vrm_name(&self, vrm_entity: Option<Entity>) -> ApiResult<Option<Name>> {
        self.0
            .schedule(move |task| async move {
                match vrm_entity {
                    Some(entity) => {
                        let vrm_name = task
                            .will(Update, once::run(obtain_vrm_name).with(entity))
                            .await?;
                        Ok(Some(vrm_name))
                    }
                    None => Ok(None),
                }
            })
            .await?
    }
}

fn load_option<V: DeserializeOwned>(
    In((vrm_name, key, default_value)): In<(Option<Name>, &'static str, V)>,
    prefs: NonSend<PrefsDatabase>,
) -> V {
    prefs
        .load_as::<V>(&gpt_key(vrm_name, key))
        .unwrap_or(default_value)
}

fn save_option<V: Serialize>(
    In((vrm_name, key, value)): In<(Option<Name>, &'static str, V)>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult {
    prefs
        .save(&gpt_key(vrm_name, key), &value)
        .map_err(|e| ApiError::FailedSave(e.to_string()))
}

fn gpt_key(vrm_name: Option<Name>, key: &str) -> String {
    match vrm_name {
        Some(name) => format!("gpt::vrm::{name}::{key}"),
        None => format!("gpt::global::{key}"),
    }
}
