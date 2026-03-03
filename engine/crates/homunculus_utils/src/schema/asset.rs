use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Type of mod asset.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Vrm,
    Vrma,
    Sound,
    Image,
    Html,
}

/// A single declared asset entry from a MOD's package.json.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AssetDeclaration {
    /// File path relative to the MOD root directory.
    pub path: String,
    /// Asset type.
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    /// Human-readable description (used by LLM agents for asset selection).
    #[serde(default)]
    pub description: Option<String>,
}

/// A resolved asset entry with full path information.
#[derive(Debug, Clone)]
pub struct AssetEntry {
    /// The asset ID (key from package.json homunculus.assets).
    pub id: AssetId,
    /// File path relative to the MOD root directory.
    pub path: PathBuf,
    /// Absolute asset path.
    pub absolute_path: PathBuf,
    /// Asset type.
    pub asset_type: AssetType,
    /// Human-readable description.
    pub description: Option<String>,
    /// The MOD name this asset belongs to.
    pub mod_name: String,
}

/// A strongly-typed asset identifier.
///
/// Wraps a `String` but provides type safety so arbitrary strings cannot be
/// passed where an asset ID is expected. Serializes transparently as a plain
/// JSON string so the HTTP/JSON API schema is unchanged.
#[repr(transparent)]
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(transparent)]
#[derive(Default)]
pub struct AssetId(String);

impl AssetId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::ops::Deref for AssetId {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AssetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for AssetId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for AssetId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl std::borrow::Borrow<str> for AssetId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
