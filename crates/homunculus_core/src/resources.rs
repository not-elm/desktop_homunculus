use crate::prelude::WebviewOpenOptions;
use bevy::prelude::*;
use std::path::PathBuf;

pub mod prelude {
    pub use crate::resources::{ModMenuMetadata, ModMenuMetadataList};
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ModMenuMetadataList(pub Vec<ModMenuMetadata>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ModMenuMetadata {
    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,
    pub text: String,
    /// The script path relative to the `assets/mods` directory.
    pub script: Option<PathBuf>,
    #[serde(rename = "webview")]
    pub webview: Option<WebviewOpenOptions>,
}

pub(crate) struct CoreResourcesPlugin;

impl Plugin for CoreResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModMenuMetadataList>();
    }
}
