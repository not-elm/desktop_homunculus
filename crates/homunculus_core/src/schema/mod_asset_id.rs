use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::path::PathBuf;
use url::Url;

/// Generally, the module path is a relative path from the `mods` directory.
/// For example, if you want to refer to `index.js` in the following directory structure, the module path would be `example/index.js`.
///
/// ```text
/// .
/// └── assets
///     └── mods
///         └── example
///             └── index.js
/// ```
///
///
/// Although remote URLs are supported, currently only webviews actually work with them.
#[repr(transparent)]
#[derive(
    Reflect, Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Default, Component,
)]
#[reflect(Serialize, Deserialize)]
pub struct ModModuleSource(pub String);

impl ModModuleSource {
    /// Convert to a resource specifier that can be used in the MOD system.
    ///
    /// If the URL is a valid HTTP or HTTPS URL, it will return a `ModModuleSpecifier::Remote`.
    /// Otherwise, it will return a `ModModuleSpecifier::Local` with the path converted to an asset path.
    pub fn to_specifier(&self) -> ModModuleSpecifier {
        if let Ok(uri) = Url::parse(&self.0)
            && (uri.scheme() == "http" || uri.scheme() == "https")
        {
            ModModuleSpecifier::Remote(uri)
        } else if self.0.starts_with("mods/") {
            ModModuleSpecifier::Local(PathBuf::from(&self.0))
        } else {
            ModModuleSpecifier::Local(PathBuf::from("mods").join(&self.0))
        }
    }
}

impl std::fmt::Display for ModModuleSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.to_specifier() {
                ModModuleSpecifier::Remote(url) => url.to_string(),
                ModModuleSpecifier::Local(path) => path.to_string_lossy().to_string(),
            }
        )
    }
}

/// Represents a module specifier for MODs.
/// Currently, only webviews support this specifier.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ModModuleSpecifier {
    Remote(Url),
    Local(PathBuf),
}
