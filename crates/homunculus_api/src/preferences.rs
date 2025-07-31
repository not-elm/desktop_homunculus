use crate::api;
use crate::error::{ApiError, ApiResult};
use bevy::app::Update;
use bevy::prelude::{In, NonSend, Transform};
use bevy_flurx::prelude::once;
use homunculus_prefs::{PrefsDatabase, PrefsKeys};
use serde::de::DeserializeOwned;

api!(PrefsApi);

impl PrefsApi {
    /// Load a preference by its key.
    pub async fn load(&self, key: String) -> ApiResult<serde_json::Value> {
        let k = key.clone();
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(load_preference).with(k)).await
            })
            .await?
            .ok_or_else(|| ApiError::NotfoundPreferences(key))
    }

    /// Load a preference and deserialize it into the specified type.
    pub async fn load_as<V: DeserializeOwned>(&self, key: String) -> ApiResult<V> {
        let value = self.load(key).await?;
        serde_json::from_value(value).map_err(|e| ApiError::FailedLoad(e.to_string()))
    }

    /// Save a preference with the specified key and value.
    pub async fn save(&self, key: String, value: serde_json::Value) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(save_preference).with((key, value)))
                    .await
            })
            .await?
            .map_err(|e| ApiError::FailedSave(e.to_string()))
    }

    /// Load a VRM transform preference by its name.
    pub async fn load_vrm_transform(&self, vrm_name: &str) -> Transform {
        self.load_as(PrefsKeys::vrm_transform(vrm_name))
            .await
            .unwrap_or_default()
    }

    /// Save a VRM transform preference with the specified name and transform.
    pub async fn save_vrm_transform(&self, vrm_name: &str, value: Transform) -> ApiResult {
        self.save(
            PrefsKeys::vrm_transform(vrm_name),
            serde_json::to_value(value).unwrap_or_default(),
        )
        .await
    }
}

fn load_preference(
    In(key): In<String>,
    preferences: NonSend<PrefsDatabase>,
) -> Option<serde_json::Value> {
    preferences.load(&key)
}

fn save_preference(
    In((key, value)): In<(String, serde_json::Value)>,
    preferences: NonSend<PrefsDatabase>,
) -> Result<(), String> {
    preferences.save(&key, &value).map_err(|e| e.to_string())
}
