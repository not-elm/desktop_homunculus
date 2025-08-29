use crate::prelude::{ModModuleSource, TransformArgs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WebviewOpenOptions {
    /// The source of the webview, which can be a URL or a local file path(Relative to `assets/mods` dir).
    pub source: ModModuleSource,
    /// Specifying this is optional, but it can be useful for tracking purposes.
    /// If you don't specify this, the webview will not be associated with any specific VRM.
    pub vrm: Option<u64>,
    pub parent: Option<u64>,
    pub transform: Option<TransformArgs>,
    /// If specified, when the webview is opened,
    /// it sounds the specified sound.
    pub sounds: Option<WebviewSoundOptions>,
}

/// The options to set interactive sounds for the webview.
///
/// Every sound path is relative to the `assets/mods` directory.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebviewSoundOptions {
    /// The sound local path to play when the webview is opened.
    pub open: Option<ModModuleSource>,
    /// The sound local path to play when the webview is closed.
    pub close: Option<ModModuleSource>,
}
