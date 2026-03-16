use crate::rpc_registry::SharedRpcRegistry;
use bevy::prelude::*;
use crate::rpc_registry::SharedRpcRegistry;

pub mod prelude {
    pub use crate::resources::{ModMenuMetadata, ModMenuMetadataList};
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct ModMenuMetadataList(pub Vec<ModMenuMetadata>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ModMenuMetadata {
    pub id: String,
    pub mod_name: String,
    pub text: String,
    pub command: String,
}

pub(crate) struct CoreResourcesPlugin;

impl Plugin for CoreResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ModMenuMetadataList>();
        app.init_resource::<SharedRpcRegistry>();
    }
}
