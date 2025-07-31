use crate::prelude::ModModuleSource;
use bevy::math::{IVec2, Vec2};
use bevy_vrm1::prelude::VrmBone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WebviewOpenOptions {
    /// The source of the webview, which can be a URL or a local file path(Relative to `assets/mods` dir).
    pub source: ModModuleSource,
    /// Specifying this is optional, but it can be useful for tracking purposes.
    /// If you don't specify this, the webview will not be associated with any specific VRM.
    pub caller: Option<u64>,
    /// If true, the webview will be opened in a transparent window.
    pub transparent: Option<bool>,
    /// If true, it displays the toolbar at the top of the webview.
    ///
    /// If not specified, the toolbar will be shown.
    #[serde(rename = "showToolbar")]
    pub show_toolbar: Option<bool>,
    /// If true, the window will have a drop shadow.
    ///
    /// This is only effective on macOS.
    pub shadow: Option<bool>,
    /// If specified, the webview will be opened at the specified position.
    ///
    /// If not specified, the webview will be opened at the center of the primary window.
    pub position: Option<WebviewOpenPosition>,
    /// The window resolution.
    pub resolution: Option<Vec2>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum WebviewOpenPosition {
    /// Opens the webview at the specified position in screen coordinate.
    At(IVec2),

    /// Opens the webview at the position relative to the specified the VRM avatar.
    Vrm {
        /// The VRM entity id to which the webview is associated.
        vrm: Option<u64>,
        /// The bone of VRM to which the webview is associated.
        bone: Option<VrmBone>,
        /// The offset from the VRM position(Screen Coordinates).
        ///
        /// If the bone is specified, this will be the offset from the bone's position.
        offset: Option<IVec2>,
        /// If true, the webview will be tracking the VRM avatar's position.
        tracking: Option<bool>,
    },
}

impl Default for WebviewOpenPosition {
    fn default() -> Self {
        WebviewOpenPosition::At(IVec2::ZERO)
    }
}
