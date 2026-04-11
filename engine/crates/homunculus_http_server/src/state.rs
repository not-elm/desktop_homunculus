use axum::extract::FromRef;
use homunculus_api::assets::AssetsApi;
use homunculus_api::mods::ModsApi;
use homunculus_api::persona::PersonaApi;
use homunculus_api::preferences::PrefsApi;
use homunculus_api::processes::ProcessesApi;
use homunculus_api::prelude::{
    ApiReactor, AppApi, AudioBgmApi, AudioSeApi, CameraApi, EffectsApi, EntitiesApi, SettingsApi,
    ShadowPanelApi, SignalsApi, SpeechApi, VrmAnimationApi, WebviewApi,
};
use homunculus_api::stt::SttApi;
use homunculus_api::vrm::VrmApi;
use homunculus_core::rpc_registry::RpcRegistry;
use homunculus_utils::config::HomunculusConfig;
use std::sync::{Arc, RwLock};

#[derive(Clone, FromRef)]
pub struct HttpState {
    pub reactor: ApiReactor,
    pub app: AppApi,
    pub audio_se: AudioSeApi,
    pub audio_bgm: AudioBgmApi,
    pub persona: PersonaApi,
    pub vrm: VrmApi,
    pub vrma: VrmAnimationApi,
    pub prefs: PrefsApi,
    pub camera: CameraApi,
    pub settings: SettingsApi,
    pub shadow_panel: ShadowPanelApi,
    pub webview: WebviewApi,
    pub effects: EffectsApi,
    pub speak: SpeechApi,
    pub signals: SignalsApi,
    pub entities: EntitiesApi,
    pub assets: AssetsApi,
    pub mods: ModsApi,
    pub processes: ProcessesApi,
    /// STT API — stateless speech recognition and model downloads.
    /// Bypasses ApiReactor; audio pipelines are managed internally.
    pub stt: SttApi,
    pub config: HomunculusConfig,
    pub rpc_registry: Arc<RwLock<RpcRegistry>>,
}

impl HttpState {
    pub fn new(
        reactor: ApiReactor,
        config: HomunculusConfig,
        rpc_registry: Arc<RwLock<RpcRegistry>>,
    ) -> Self {
        Self {
            app: AppApi::from(reactor.clone()),
            audio_se: AudioSeApi::from(reactor.clone()),
            audio_bgm: AudioBgmApi::from(reactor.clone()),
            persona: PersonaApi::from(reactor.clone()),
            vrm: VrmApi::from(reactor.clone()),
            vrma: VrmAnimationApi::from(reactor.clone()),
            prefs: PrefsApi::from(reactor.clone()),
            camera: CameraApi::from(reactor.clone()),
            settings: SettingsApi::from(reactor.clone()),
            shadow_panel: ShadowPanelApi::from(reactor.clone()),
            webview: WebviewApi::from(reactor.clone()),
            effects: EffectsApi::from(reactor.clone()),
            speak: SpeechApi::from(reactor.clone()),
            signals: SignalsApi::from(reactor.clone()),
            entities: EntitiesApi::from(reactor.clone()),
            assets: AssetsApi::from(reactor.clone()),
            mods: ModsApi::from(reactor.clone()),
            processes: ProcessesApi::from(reactor.clone()),
            stt: SttApi::new(reactor.clone()),
            config,
            rpc_registry,
            reactor,
        }
    }
}
