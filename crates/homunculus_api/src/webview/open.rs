use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::close::ClosingWebviewSounds;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_cef::prelude::{CefWebviewUri, PreloadScripts, WebviewExtendStandardMaterial};
use bevy_flurx::action::once;
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::{ModModuleSpecifier, WebviewOpenOptions, WebviewSoundOptions};
use homunculus_effects::{Entity, Update};

impl WebviewApi {
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

pub(crate) struct WebviewOpenPlugin;

impl Plugin for WebviewOpenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visible);
    }
}

fn open(
    In(options): In<WebviewOpenOptions>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WebviewExtendStandardMaterial>>,
    asset_server: Res<AssetServer>,
    cameras: Cameras,
) -> Entity {
    let webview_uri = match options.source.to_specifier() {
        ModModuleSpecifier::Remote(url) => CefWebviewUri::new(url),
        ModModuleSpecifier::Local(path) => CefWebviewUri::local(path.display().to_string()),
    };
    let webview = commands
        .spawn((
            Name::new("Webview"),
            webview_uri,
            cameras.all_layers(),
            NotShadowCaster,
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
            Visibility::Hidden,
            options
                .transform
                .as_ref()
                .map(|t| t.as_transform())
                .unwrap_or(Transform::from_xyz(0.0, 0.0, 10.0)),
        ))
        .id();

    insert_preload_scripts(&mut commands, webview, &options);
    feed_sound_options(
        webview,
        &mut commands,
        &mut ClosingWebviewSounds::default(),
        &asset_server,
        options.sounds.as_ref(),
    );
    set_parent(&mut commands, webview, &options);
    webview
}

fn visible(
    mut webviews: Query<(
        &mut Visibility,
        &MeshMaterial3d<WebviewExtendStandardMaterial>,
    )>,
    materials: Res<Assets<WebviewExtendStandardMaterial>>,
) {
    for (mut visibility, handle) in webviews.iter_mut() {
        if matches!(*visibility, Visibility::Hidden)
            && let Some(material) = materials.get(handle)
            && material.extension.surface.is_some()
        {
            *visibility = Visibility::Visible;
        }
    }
}

fn insert_preload_scripts(commands: &mut Commands, webview: Entity, options: &WebviewOpenOptions) {
    commands.entity(webview).insert(PreloadScripts::from([
        include_str!("../webview/webview.js"),
        include_str!("../webview/api.js"),
        &include_str!("../webview/vrm.js").replace(
            "undefined",
            &options
                .vrm
                .map(|s| s.to_string())
                .unwrap_or_else(|| "undefined".to_string()),
        ),
        &include_str!("../webview/webviewEntity.js")
            .replace("undefined", &webview.to_bits().to_string()),
    ]));
}

fn set_parent(commands: &mut Commands, webview: Entity, options: &WebviewOpenOptions) {
    if let Some(parent) = options.parent {
        let parent = Entity::from_bits(parent);
        commands.entity(parent).add_child(webview);
    }
}

fn feed_sound_options(
    webview_entity: Entity,
    commands: &mut Commands,
    closing_sounds: &mut ClosingWebviewSounds,
    asset_server: &AssetServer,
    options: Option<&WebviewSoundOptions>,
) {
    let Some(options) = options else {
        return;
    };
    if let Some(sound) = options.open.as_ref() {
        commands.spawn(AudioPlayer::new(asset_server.load(sound.to_string())));
    }
    if let Some(sound) = options.close.clone() {
        closing_sounds.0.insert(webview_entity, sound);
    }
}
