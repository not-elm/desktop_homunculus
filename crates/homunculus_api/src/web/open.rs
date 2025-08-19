use crate::error::ApiResult;
use crate::web::WebApi;
use bevy::prelude::*;
use bevy_cef::prelude::{CefWebviewUri, WebviewExtendStandardMaterial};
use bevy_flurx::action::once;
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::{
    ModModuleSource, ModModuleSpecifier, WebviewOpenOptions, WebviewOpenPosition,
    WebviewSoundOptions,
};
use homunculus_effects::{Entity, Update};
use serde::{Deserialize, Serialize};

impl WebApi {
    /// Opens a webview with the specified options.
    ///
    /// The webview created by this API will have `window.api` defined.
    pub async fn open(&self, options: WebviewOpenOptions) -> ApiResult<Entity> {
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(open).with(options)).await },
            )
            .await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WebOpenOptions {
    /// The source of the webview, which can be a URL or a local file path(Relative to `assets/mods` dir).
    pub source: ModModuleSource,
    /// Specifying this is optional, but it can be useful for tracking purposes.
    /// If you don't specify this, the webview will not be associated with any specific VRM.
    pub parent: Option<u64>,
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

fn open(
    In(options): In<WebviewOpenOptions>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WebviewExtendStandardMaterial>>,
    cameras: Cameras,
) -> Entity {
    let webview_uri = match options.source.to_specifier() {
        ModModuleSpecifier::Remote(url) => CefWebviewUri::new(url),
        ModModuleSpecifier::Local(path) => CefWebviewUri::local(path.display().to_string()),
    };
    let webview = commands
        .spawn((
            webview_uri,
            cameras.all_layers(),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::ONE))),
            MeshMaterial3d(materials.add(WebviewExtendStandardMaterial {
                base: StandardMaterial {
                    unlit: true,
                    #[cfg(target_os = "macos")]
                    alpha_mode: AlphaMode::Premultiplied,
                    ..default()
                },
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 10.0),
        ))
        .id();
    if let Some(caller) = options.caller {
        let vrm = Entity::from_bits(caller);
        commands.entity(vrm).add_child(webview);
    }
    webview
}
