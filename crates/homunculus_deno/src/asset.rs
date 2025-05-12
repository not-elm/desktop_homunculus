use bevy::app::Plugin;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, Handle, LoadContext};
use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(unused)]
pub struct DenoScriptHandle(pub Handle<DenoScript>);

/// The `ScriptAsset` is used to identify script files like `.ts` or `.js` as assets and is utilized for hot reloading.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct DenoScript(pub(crate) String);

pub(super) struct DenoAssetPlugin;

impl Plugin for DenoAssetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_asset::<DenoScript>()
            .init_asset_loader::<ScriptAssetLoader>();
    }
}

#[derive(Default)]
struct ScriptAssetLoader;

impl AssetLoader for ScriptAssetLoader {
    type Asset = DenoScript;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        _: &mut LoadContext<'_>,
    ) -> std::result::Result<Self::Asset, Self::Error> {
        let mut script = String::new();
        reader.read_to_string(&mut script).await?;
        Ok(DenoScript(script))
    }

    fn extensions(&self) -> &[&str] {
        &["ts", "js"]
    }
}
