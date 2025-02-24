use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, Handle, LoadContext};
use bevy::gltf::{Gltf, GltfError, GltfLoader, GltfLoaderSettings};
use bevy::image::CompressedImageFormats;
use bevy::prelude::{AssetApp, Component, TypePath};
use bevy::render::renderer::RenderDevice;
use bevy::utils::default;

pub struct VrmLoaderPlugin;

impl Plugin for VrmLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .preregister_asset_loader::<VrmLoader>(&["vrm"]);
    }

    fn finish(&self, app: &mut App) {
        let supported_compressed_formats = match app.world().get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),
            None => CompressedImageFormats::NONE,
        };
        app.register_asset_loader(VrmLoader(GltfLoader {
            supported_compressed_formats,
            custom_vertex_attributes: Default::default(),
        }));
    }
}

#[derive(Debug, Component)]
pub struct VrmHandle(pub Handle<Vrm>);

#[derive(Debug, Asset, TypePath)]
pub struct Vrm {
    pub gltf: Gltf,
}
pub struct VrmLoader(GltfLoader);

impl AssetLoader for VrmLoader {
    type Asset = Vrm;
    type Settings = ();
    type Error = GltfError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let settings = GltfLoaderSettings {
            include_source: true,
            ..default()
        };
        let gltf = self.0.load(reader, &settings, load_context).await?;
        Ok(Vrm {
            gltf,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vrm"]
    }
}

