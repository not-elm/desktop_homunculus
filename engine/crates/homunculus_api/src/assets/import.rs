//! Asset import logic.
//!
//! Copies a local file into `~/.homunculus/assets/`, registers it in the
//! [`AssetRegistry`], and persists the record to the `imported_assets` DB table.

use crate::assets::AssetsApi;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{AssetEntry, AssetId, AssetRegistry, AssetType};
use homunculus_prefs::PrefsDatabase;
use homunculus_utils::path::homunculus_dir;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Request body for `POST /assets/import`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ImportAsset {
    /// Absolute path to the source file on the local filesystem.
    pub source_path: String,
    /// Unique asset identifier (e.g. `"vrm:local:my-persona"`).
    pub asset_id: String,
    /// Asset type (`vrm`, `vrma`, `sound`, `image`, `html`).
    pub asset_type: AssetType,
    /// Optional human-readable description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Response body for `POST /assets/import`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ImportAssetResponse {
    pub asset_id: AssetId,
}

impl AssetsApi {
    /// Imports a local file as an asset.
    ///
    /// Validates the source path, copies the file to managed storage,
    /// registers it in the ECS `AssetRegistry`, and persists the record.
    pub async fn import(&self, args: ImportAsset) -> ApiResult<ImportAssetResponse> {
        let source = validate_source(&args.source_path)?;
        let dest = copy_to_managed_storage(&source, &args.asset_id)?;

        let dest_str = dest.to_string_lossy().to_string();
        let source_str = args.source_path.clone();

        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(register_and_persist).with((args, dest_str, source_str)),
                )
                .await
            })
            .await?
    }
}

/// Validates that the source file exists and is a file.
fn validate_source(source_path: &str) -> ApiResult<PathBuf> {
    let path = PathBuf::from(source_path);
    if !path.exists() {
        return Err(ApiError::InvalidInput(format!(
            "Source file does not exist: {source_path}"
        )));
    }
    if !path.is_file() {
        return Err(ApiError::InvalidInput(format!(
            "Source path is not a file: {source_path}"
        )));
    }
    Ok(path)
}

/// Copies the source file into `~/.homunculus/assets/`, preserving the
/// original filename. Creates the directory if it does not exist.
fn copy_to_managed_storage(source: &Path, asset_id: &str) -> ApiResult<PathBuf> {
    let assets_dir = homunculus_dir().join("assets");
    std::fs::create_dir_all(&assets_dir)
        .map_err(|e| ApiError::InvalidInput(format!("Failed to create assets directory: {e}")))?;

    let filename = derive_filename(source, asset_id);
    let dest = assets_dir.join(filename);

    std::fs::copy(source, &dest).map_err(|e| {
        ApiError::InvalidInput(format!("Failed to copy file to {}: {e}", dest.display()))
    })?;

    Ok(dest)
}

/// Derives a destination filename from the asset ID and the source file's
/// extension. Colons in the asset ID are replaced with underscores.
fn derive_filename(source: &Path, asset_id: &str) -> String {
    let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("");
    let stem = asset_id.replace(':', "_");
    if ext.is_empty() {
        stem
    } else {
        format!("{stem}.{ext}")
    }
}

/// Bevy one-shot system: registers the asset in `AssetRegistry` and persists
/// the record in the `imported_assets` table.
fn register_and_persist(
    In((args, dest_path, source_path)): In<(ImportAsset, String, String)>,
    mut registry: ResMut<AssetRegistry>,
    prefs: NonSend<PrefsDatabase>,
) -> ApiResult<ImportAssetResponse> {
    let asset_id = AssetId::new(&args.asset_id);
    let dest = PathBuf::from(&dest_path);

    let entry = AssetEntry {
        id: asset_id.clone(),
        path: dest.clone(),
        absolute_path: dest,
        asset_type: args.asset_type.clone(),
        description: args.description.clone(),
        mod_name: "local".to_string(),
    };
    registry.register_imported(entry);

    let type_str = serialize_asset_type(&args.asset_type);
    prefs
        .upsert_imported_asset(
            &args.asset_id,
            None,
            &dest_path,
            &type_str,
            args.description.as_deref(),
            Some(&source_path),
        )
        .map_err(|e| ApiError::Sql(e.to_string()))?;

    Ok(ImportAssetResponse { asset_id })
}

/// Serializes an `AssetType` to its lowercase string representation.
fn serialize_asset_type(asset_type: &AssetType) -> String {
    serde_json::to_value(asset_type)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_filename_replaces_colons() {
        let source = PathBuf::from("/tmp/model.vrm");
        assert_eq!(
            derive_filename(&source, "vrm:local:my-persona"),
            "vrm_local_my-persona.vrm"
        );
    }

    #[test]
    fn derive_filename_no_extension() {
        let source = PathBuf::from("/tmp/model");
        assert_eq!(derive_filename(&source, "vrm:local:test"), "vrm_local_test");
    }
}
