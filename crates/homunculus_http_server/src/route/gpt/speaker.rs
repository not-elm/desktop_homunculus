use crate::route::gpt::{GptQuery, to_entity};
use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::prelude::GptApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::{Deserialize, Serialize};

/// Get the voice vox speaker ID used to read out GPT messages.
///
/// ### Path
///
/// `GET /gpt/speaker/voicevox`
///
/// ### Queries
/// - `vrm`: Optional VRM entity ID to check specific settings for that VRM.
pub async fn get(State(api): State<GptApi>, Query(query): Query<GptQuery>) -> HttpResult<u32> {
    api.voicevox_speaker(query.vrm_entity())
        .await
        .into_http_result()
}

/// Save the voice vox speaker ID used to read out GPT messages.
///
/// ### Path
///
/// `PUT /gpt/speaker/voicevox`
pub async fn put(
    State(api): State<GptApi>,
    Json(request): Json<PutVoiceVoxSpeakerRequest>,
) -> HttpResult<()> {
    api.save_voicevox_speaker(request.id, to_entity(request.vrm))
        .await
        .into_http_result()
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PutVoiceVoxSpeakerRequest {
    /// The voicevox speaker ID to set for reading out GPT messages.
    pub id: u32,
    /// Optional VRM entity ID to set specific settings for that VRM.
    pub vrm: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::route::gpt::speaker::PutVoiceVoxSpeakerRequest;
    use crate::tests::{assert_response, call, test_app};
    use axum::http;
    use axum::http::Request;
    use bevy::prelude::*;
    use homunculus_api::prelude::GptApi;
    use homunculus_prefs::PrefsDatabase;

    #[tokio::test]
    async fn get_default_global_voice_vox_speaker() {
        let (mut app, router) = test_app();
        let request = Request::get("/gpt/speaker/voicevox")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, GptApi::DEFAULT_VOICEVOX_SPEAKER).await;
    }

    #[tokio::test]
    async fn get_global_voice_vox_speaker() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::global::speaker::voicevox", &0)
            .unwrap();
        let request = Request::get("/gpt/speaker/voicevox")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, 0).await;
    }

    #[tokio::test]
    async fn get_default_vrm_voice_vox_speaker() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/speaker/voicevox?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, GptApi::DEFAULT_VOICEVOX_SPEAKER).await;
    }

    #[tokio::test]
    async fn get_vrm_voicevox_speaker() {
        let (mut app, router) = test_app();
        app.world_mut()
            .non_send_resource::<PrefsDatabase>()
            .save("gpt::vrm::Avatar::speaker::voicevox", &0)
            .unwrap();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::get(format!("/gpt/speaker/voicevox?vrm={}", vrm.to_bits()))
            .body(axum::body::Body::empty())
            .unwrap();
        assert_response(&mut app, router, request, 0).await;
    }

    #[tokio::test]
    async fn put_global_model() {
        let (mut app, router) = test_app();
        let request = Request::put("/gpt/speaker/voicevox")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutVoiceVoxSpeakerRequest { id: 0, vrm: None }).unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<isize>("gpt::global::speaker::voicevox")
            .unwrap();
        assert_eq!(model, 0);
    }

    #[tokio::test]
    async fn put_vrm_voicevox_speaker() {
        let (mut app, router) = test_app();
        let vrm = app.world_mut().spawn(Name::new("Avatar")).id();
        let request = Request::put("/gpt/speaker/voicevox")
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(
                serde_json::to_string(&PutVoiceVoxSpeakerRequest {
                    id: 0,
                    vrm: Some(vrm.to_bits()),
                })
                .unwrap(),
            ))
            .unwrap();
        call(&mut app, router, request).await;

        let model = app
            .world_mut()
            .non_send_resource::<PrefsDatabase>()
            .load_as::<isize>("gpt::vrm::Avatar::speaker::voicevox")
            .unwrap();
        assert_eq!(model, 0);
    }
}
