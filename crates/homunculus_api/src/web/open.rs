use crate::error::ApiResult;
use crate::web::WebApi;
use bevy::prelude::*;
use bevy_cef::prelude::{CefWebviewUri, WebviewExtendStandardMaterial};
use bevy_flurx::action::once;
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::{ModModuleSpecifier, WebviewOpenOptions};
use homunculus_effects::{Entity, Update};

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
    commands
        .spawn((
            webview_uri,
            cameras.all_layers(),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::ONE))),
            MeshMaterial3d(materials.add(WebviewExtendStandardMaterial::default())),
            Transform::from_xyz(0.0, 0.0, 10.0),
        ))
        .id()
}
