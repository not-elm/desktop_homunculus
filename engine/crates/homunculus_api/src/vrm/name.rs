use std::collections::HashMap;

use crate::error::{ApiError, ApiResult};
use crate::vrm::VrmApi;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::AssetIdComponent;
use homunculus_prefs::prelude::{PrefsDatabase, PrefsKeys};
use serde::{Deserialize, Serialize};

/// All multilingual display names for a VRM entity.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct VrmNames {
    /// The original name from VRM metadata (the Bevy `Name` component).
    pub metadata: String,
    /// Language code → display name mappings stored in preferences.
    pub names: HashMap<String, String>,
}

/// Request body for setting a VRM display name.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SetNameRequest {
    /// The display name to set. If empty/whitespace-only, the name is deleted.
    pub name: String,
    /// BCP-47 language code (e.g. "en", "ja"). Defaults to "en".
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "en".to_string()
}

impl VrmApi {
    /// Retrieves the display name of a VRM entity for a given language.
    ///
    /// If a localized name is stored in preferences, it is returned.
    /// Otherwise, falls back to the Bevy `Name` component (VRM metadata name).
    pub async fn get_name(&self, entity: Entity, lang: String) -> ApiResult<String> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_name).with((entity, lang)))
                    .await
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }

    /// Sets or deletes the display name of a VRM entity for a given language.
    ///
    /// If `name` is empty or whitespace-only, the stored name is deleted instead.
    pub async fn set_name(&self, entity: Entity, lang: String, name: String) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_name).with((entity, lang, name)))
                    .await
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }

    /// Deletes the display name of a VRM entity for a given language.
    pub async fn delete_name(&self, entity: Entity, lang: String) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(delete_name).with((entity, lang)))
                    .await
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }

    /// Lists all multilingual display names of a VRM entity.
    ///
    /// Returns the VRM metadata name and all language-specific names stored in preferences.
    pub async fn list_names(&self, entity: Entity) -> ApiResult<VrmNames> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(list_names).with(entity)).await
            })
            .await?
            .ok_or(ApiError::EntityNotFound)
    }
}

fn get_name(
    In((entity, lang)): In<(Entity, String)>,
    query: Query<(&Name, &AssetIdComponent)>,
    prefs: NonSend<PrefsDatabase>,
) -> Option<String> {
    let (name, asset_id) = query.get(entity).ok()?;
    let lang = lang.to_lowercase();
    let key = PrefsKeys::name(asset_id.0.as_ref(), &lang);
    if let Ok(Some(stored)) = prefs.load_as::<String>(&key) {
        Some(stored)
    } else {
        Some(name.to_string())
    }
}

fn set_name(
    In((entity, lang, name)): In<(Entity, String, String)>,
    query: Query<&AssetIdComponent>,
    prefs: NonSend<PrefsDatabase>,
) -> Option<()> {
    let asset_id = query.get(entity).ok()?;
    let lang = lang.to_lowercase();
    let key = PrefsKeys::name(asset_id.0.as_ref(), &lang);
    if name.trim().is_empty() {
        let _ = prefs.delete(&key);
    } else {
        let _ = prefs.save_as(&key, &name);
    }
    Some(())
}

fn delete_name(
    In((entity, lang)): In<(Entity, String)>,
    query: Query<&AssetIdComponent>,
    prefs: NonSend<PrefsDatabase>,
) -> Option<()> {
    let asset_id = query.get(entity).ok()?;
    let lang = lang.to_lowercase();
    let key = PrefsKeys::name(asset_id.0.as_ref(), &lang);
    let _ = prefs.delete(&key);
    Some(())
}

fn list_names(
    In(entity): In<Entity>,
    query: Query<(&Name, &AssetIdComponent)>,
    prefs: NonSend<PrefsDatabase>,
) -> Option<VrmNames> {
    let (name, asset_id) = query.get(entity).ok()?;
    let metadata = name.to_string();
    let prefix = format!("name::{}::", asset_id.0.as_ref());

    let mut names = HashMap::new();
    if let Ok(keys) = prefs.list_keys() {
        for key in keys {
            if let Some(lang) = key.strip_prefix(&prefix)
                && let Ok(Some(value)) = prefs.load_as::<String>(&key)
            {
                names.insert(lang.to_string(), value);
            }
        }
    }

    Some(VrmNames { metadata, names })
}
