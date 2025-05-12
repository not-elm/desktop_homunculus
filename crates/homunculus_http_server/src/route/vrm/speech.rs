use crate::extract::EntityId;
use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_api::prelude::{SpeakVoiceVoxOptions, SpeechApi};
use serde::{Deserialize, Serialize};

/// Speak using VoiceVox.
///
/// ### Path
///
/// `POST /vrm/:entity_id/speech/voicevox`
pub async fn voicevox(
    State(api): State<SpeechApi>,
    EntityId(vrm): EntityId,
    Json(body): Json<VoiceVoxBody>,
) -> HttpResult {
    api.speak_on_voicevox(vrm, body.sentences, body.options.unwrap_or_default())
        .await
        .into_http_result()
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VoiceVoxBody {
    pub sentences: Vec<String>,
    #[serde(flatten)]
    pub options: Option<SpeakVoiceVoxOptions>,
}
