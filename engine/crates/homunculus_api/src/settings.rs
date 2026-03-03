use crate::error::ApiResult;
use crate::prelude::{ApiReactor, PrefsApi};
use bevy::prelude::{Commands, In, Update};
use bevy_flurx::action::once;
use homunculus_power_saver::{MAX_FPS_KEY, RequestUpdateFrameRate};

#[derive(Clone, bevy::prelude::Resource)]
pub struct SettingsApi(PrefsApi);

impl From<ApiReactor> for SettingsApi {
    fn from(reactor: ApiReactor) -> Self {
        Self(PrefsApi::from(reactor))
    }
}

impl SettingsApi {
    pub const MAX_FPS: &'static str = MAX_FPS_KEY;

    pub async fn fps(&self) -> f64 {
        self.0
            .load(Self::MAX_FPS.to_string())
            .await
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(60.0)
    }

    pub async fn set_fps(&self, fps: f64) -> ApiResult {
        self.0
            .save(Self::MAX_FPS.to_string(), serde_json::Value::from(fps))
            .await?;
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(request_update_frame_rate).with(fps))
                    .await;
            })
            .await?;
        Ok(())
    }
}

fn request_update_frame_rate(In(fps): In<f64>, mut commands: Commands) {
    commands.trigger(RequestUpdateFrameRate(fps));
}
