use crate::api;
use crate::error::{ApiError, ApiResult};
use bevy::app::Update;
use bevy::prelude::{In, NonSend};
use bevy_flurx::prelude::once;
use homunculus_prefs::PrefsDatabase;
use serde::de::DeserializeOwned;

api!(PrefsApi);

impl PrefsApi {
    /// List all saved preference keys.
    pub async fn list(&self) -> ApiResult<Vec<String>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_keys)).await })
            .await?
            .map_err(ApiError::Sql)
    }

    /// Load a preference by its key.
    pub async fn load(&self, key: String) -> ApiResult<serde_json::Value> {
        let k = key.clone();
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(load_preference).with(k)).await
            })
            .await?
            .map_err(ApiError::InvalidInput)?
            .ok_or_else(|| ApiError::NotFoundPreferences(key))
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
}

fn list_keys(preferences: NonSend<PrefsDatabase>) -> Result<Vec<String>, String> {
    preferences.list_keys().map_err(|e| e.to_string())
}

fn load_preference(
    In(key): In<String>,
    preferences: NonSend<PrefsDatabase>,
) -> Result<Option<serde_json::Value>, String> {
    preferences.load_json(&key).map_err(|e| e.to_string())
}

fn save_preference(
    In((key, value)): In<(String, serde_json::Value)>,
    preferences: NonSend<PrefsDatabase>,
) -> Result<(), String> {
    preferences
        .save_json(&key, &value)
        .map_err(|e| e.to_string())
}
