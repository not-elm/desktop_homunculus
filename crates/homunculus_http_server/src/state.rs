use axum::extract::FromRef;
use homunculus_api::preferences::PrefsApi;
use homunculus_api::prelude::{
    ApiReactor, AppApi, CameraApi, CommandsApi, EffectsApi, EntitiesApi, GptApi, ModsApi,
    ScriptsApi, SettingsApi, ShadowPanelApi, SpeechApi, VrmAnimationApi, WebviewApi,
};
use homunculus_api::vrm::VrmApi;

#[derive(Clone, FromRef)]
pub struct HttpState {
    pub reactor: ApiReactor,
    pub app: AppApi,
    pub vrm: VrmApi,
    pub vrma: VrmAnimationApi,
    pub prefs: PrefsApi,
    pub camera: CameraApi,
    pub shadow_panel: ShadowPanelApi,
    pub webview: WebviewApi,
    pub effects: EffectsApi,
    pub speak: SpeechApi,
    pub script: ScriptsApi,
    pub settings: SettingsApi,
    pub gpt: GptApi,
    pub mods: ModsApi,
    pub commands: CommandsApi,
    pub entities: EntitiesApi,
}

impl From<ApiReactor> for HttpState {
    fn from(reactor: ApiReactor) -> Self {
        Self {
            app: AppApi::from(reactor.clone()),
            vrm: VrmApi::from(reactor.clone()),
            vrma: VrmAnimationApi::from(reactor.clone()),
            prefs: PrefsApi::from(reactor.clone()),
            camera: CameraApi::from(reactor.clone()),
            shadow_panel: ShadowPanelApi::from(reactor.clone()),
            webview: WebviewApi::from(reactor.clone()),
            effects: EffectsApi::from(reactor.clone()),
            speak: SpeechApi::from(reactor.clone()),
            script: ScriptsApi::from(reactor.clone()),
            settings: SettingsApi::from(reactor.clone()),
            gpt: GptApi::from(reactor.clone()),
            mods: ModsApi::from(reactor.clone()),
            commands: CommandsApi::from(reactor.clone()),
            entities: EntitiesApi::from(reactor.clone()),
            reactor,
        }
    }
}
