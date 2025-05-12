use crate::voicevox::RequestSpeak;
use crate::voicevox::audio_query::AudioQuery;
use crate::{Mora, Speak, SpeakQueue};
use async_channel::{Receiver, Sender};
use async_compat::CompatExt;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use homunculus_core::prelude::OutputLog;

#[derive(Resource)]
struct RequestSpeakSender(Sender<(Entity, RequestSpeak)>);

#[derive(Resource)]
struct RequestSpeakReceiver(Receiver<(Entity, RequestSpeak)>);

#[derive(Resource)]
struct SpeakSender(Sender<(Entity, Speak)>);

#[derive(Resource)]
struct SpeakReceiver(Receiver<(Entity, Speak)>);

pub(super) struct VoiceVoxLoaderPlugin;

impl Plugin for VoiceVoxLoaderPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = async_channel::unbounded();
        let (speak_tx, speak_rx) = async_channel::unbounded();
        app.insert_resource(RequestSpeakSender(tx))
            .insert_resource(RequestSpeakReceiver(rx))
            .insert_resource(SpeakSender(speak_tx))
            .insert_resource(SpeakReceiver(speak_rx))
            .add_systems(Startup, handle_request_speech_queue)
            .add_systems(Update, handle_speak_queue)
            .add_observer(load);
    }
}

fn load(trigger: Trigger<RequestSpeak>, sender: Res<RequestSpeakSender>) {
    let sender = sender.0.clone();
    let vrm_entity = trigger.target();
    let speak = trigger.event().clone();
    IoTaskPool::get()
        .spawn(async move {
            let _ = sender.send((vrm_entity, speak)).await;
        })
        .detach();
}

fn handle_request_speech_queue(rx: Res<RequestSpeakReceiver>, tx: Res<SpeakSender>) {
    let rx = rx.0.clone();
    let tx = tx.0.clone();
    IoTaskPool::get()
        .spawn(
            async move {
                while let Ok((vrm_entity, speak)) = rx.recv().await {
                    for (i, sentence) in speak.sentences.iter().enumerate() {
                        let Ok(query) = fetch_audio_query(speak.speaker, sentence).await else {
                            continue;
                        };
                        let Ok(wav) = fetch_synthesis(&query, speak.speaker).await else {
                            continue;
                        };
                        let mut moras = query.to_moras();
                        if let Some(pause) = speak.pause {
                            moras.push_back(Mora {
                                timer: Timer::new(pause, TimerMode::Once),
                                vowel: None,
                            });
                        }
                        let finish_signal = if i == speak.sentences.len() - 1 {
                            speak.finish_signal.clone()
                        } else {
                            None
                        };
                        tx.send((
                            vrm_entity,
                            Speak {
                                text: sentence.clone(),
                                moras,
                                wav,
                                subtitle: speak.subtitle.clone(),
                                finish_signal,
                            },
                        ))
                        .await
                        .output_log_if_error("handle_speech_queue:send");
                    }
                }
            }
            .compat(),
        )
        .detach();
}

fn handle_speak_queue(rx: Res<SpeakReceiver>, mut vrms: Query<&mut SpeakQueue>) {
    while let Ok((vrm_entity, speak)) = rx.0.try_recv() {
        if let Ok(mut queue) = vrms.get_mut(vrm_entity) {
            queue.push_back(speak);
        }
    }
}

async fn fetch_audio_query(speaker: u32, text: &str) -> reqwest::Result<AudioQuery> {
    reqwest::Client::new()
        .post(format!(
            "http://localhost:50021/audio_query?speaker={speaker}&text={text}",
        ))
        .send()
        .await?
        .json()
        .await
}

async fn fetch_synthesis(query: &AudioQuery, speaker: u32) -> reqwest::Result<Vec<u8>> {
    Ok(reqwest::Client::new()
        .post(format!(
            "http://localhost:50021/synthesis?speaker={speaker}"
        ))
        .json(&query)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec())
}
