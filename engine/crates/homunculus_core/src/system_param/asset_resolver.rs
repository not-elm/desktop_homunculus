use bevy::asset::{Asset, AssetServer};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::prelude::{AssetEntry, AssetRegistry};

/// Resolves asset IDs to Bevy asset handles.
///
/// Use [`load()`](AssetResolver::load) for VRM/VRMA/Effects (returns a Handle).
/// Use [`resolve()`](AssetResolver::resolve) for Webviews (returns the AssetEntry for path resolution).
///
/// # Example
///
/// ```ignore
/// fn my_system(resolver: AssetResolver) {
///     let handle: Handle<Scene> = resolver.load("elmer:vrm").unwrap();
/// }
/// ```
#[derive(SystemParam)]
pub struct AssetResolver<'w> {
    registry: Res<'w, AssetRegistry>,
    asset_server: Res<'w, AssetServer>,
}

impl AssetResolver<'_> {
    /// Resolve an asset ID and load it through the Bevy AssetServer.
    ///
    /// Returns a typed Handle for the asset. The asset is loaded via the
    /// `asset://` AssetSource (ModAssetReader).
    pub fn load<A: Asset>(&self, asset_id: &str) -> Result<Handle<A>, AssetResolveError> {
        Ok(self.asset_server.load(self.extract_asset_url(asset_id)?))
    }

    /// Resolve an asset ID to its registry entry without loading.
    ///
    /// Useful for Webview source resolution where Bevy AssetServer isn't used.
    pub fn resolve(&self, asset_id: &str) -> Result<&AssetEntry, AssetResolveError> {
        self.registry
            .get(asset_id)
            .ok_or_else(|| AssetResolveError::NotFound(asset_id.to_string()))
    }

    fn extract_asset_url(&self, asset_id: &str) -> Result<String, AssetResolveError> {
        let entry = self
            .registry
            .get(asset_id)
            .ok_or_else(|| AssetResolveError::NotFound(asset_id.to_string()))?;
        let asset_url = format!("asset://{}/{}", entry.mod_name, entry.path.display());
        Ok(asset_url)
    }
}

/// Error type for asset resolution failures.
#[derive(Debug)]
pub enum AssetResolveError {
    /// The asset ID was not found in the registry.
    NotFound(String),
}

impl std::fmt::Display for AssetResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetResolveError::NotFound(id) => write!(f, "Asset not found: {id}"),
        }
    }
}

impl std::error::Error for AssetResolveError {}
