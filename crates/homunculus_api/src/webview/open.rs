use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::{ClosingWebviewSounds, WebviewTracking};
use bevy::window::{WindowLevel, WindowResolution};
use bevy_flurx::action::once;
use bevy_webview_wry::core::{
    Background, BrowserAcceleratorKeys, EnableClipboard, InitializationScripts, UseDevtools,
    WebViewBundle, Webview, WebviewUri,
};
use homunculus_core::prelude::{
    BoneOffsets, Coordinate, ModModuleSpecifier, WebviewOpenOptions, WebviewOpenPosition,
    WebviewSoundOptions,
};
use homunculus_effects::{
    AssetServer, AudioPlayer, Commands, Entity, IVec2, In, Query, Res, ResMut, Transform, Update,
    Window, WindowPosition, default,
};

impl WebviewApi {
    /// Opens a webview with the specified options.
    ///
    /// The webview created by this API will have `window.api` defined.
    pub async fn open(&self, options: WebviewOpenOptions) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(open_webview).with(options))
                    .await
            })
            .await?
    }
}

fn open_webview(
    In(options): In<WebviewOpenOptions>,
    mut commands: Commands,
    mut closing_sounds: ResMut<ClosingWebviewSounds>,
    coordinate: Coordinate,
    bone_offsets: BoneOffsets,
    asset_server: Res<AssetServer>,
    transforms: Query<&Transform>,
) -> ApiResult<Entity> {
    let webview_url = match options.source.to_specifier() {
        ModModuleSpecifier::Remote(url) => WebviewUri::new(url),
        ModModuleSpecifier::Local(path) => WebviewUri::relative_local(path),
    };
    let mut window = Window::default();
    let mut background = Background::default();
    if let Some(resolution) = options.resolution {
        window.resolution = WindowResolution::new(resolution.x, resolution.y);
    }
    window.position = window_position(&options.position, &coordinate, &bone_offsets, &transforms)
        .unwrap_or_default();
    if options.transparent.is_some_and(|transparent| transparent) {
        background = Background::Transparent;
        window.transparent = true;
        window.composite_alpha_mode = bevy::window::CompositeAlphaMode::PostMultiplied;
    }
    window.has_shadow = options.shadow.unwrap_or(true);
    window.titlebar_shown = options.show_toolbar.unwrap_or(true);
    window.window_level = WindowLevel::AlwaysOnTop;
    let webview_entity = commands.spawn_empty().id();
    commands.entity(webview_entity).insert((
        WebViewBundle {
            webview: Webview::Uri(webview_url),
            background,
            enable_clipboard: EnableClipboard(true),
            use_devtools: UseDevtools(true),
            browser_accelerator_keys: BrowserAcceleratorKeys(true),
            ..default()
        },
        window,
        InitializationScripts::new(vec![
            include_str!("./webview.js"),
            include_str!("./api.js"),
            &include_str!("./caller.js").replace(
                "undefined",
                &options
                    .caller
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "undefined".to_string()),
            ),
            &include_str!("./webviewEntity.js")
                .replace("undefined", &webview_entity.to_bits().to_string()),
        ]),
    ));
    insert_tracking(&mut commands, webview_entity, &options);
    feed_sound_options(
        webview_entity,
        &mut commands,
        &mut closing_sounds,
        &asset_server,
        options.sounds.as_ref(),
    );
    Ok(webview_entity)
}

fn window_position(
    position: &Option<WebviewOpenPosition>,
    coordinate: &Coordinate,
    bone_offsets: &BoneOffsets,
    transforms: &Query<&Transform>,
) -> Option<WindowPosition> {
    match position.as_ref()? {
        WebviewOpenPosition::At(offset) => Some(WindowPosition::At(IVec2::new(offset.x, offset.y))),
        WebviewOpenPosition::Vrm {
            vrm: Some(vrm),
            bone,
            offset,
            ..
        } => {
            let vrm_entity = Entity::from_bits(*vrm);
            let tf = transforms.get(vrm_entity).ok()?;
            let world_pos = match bone {
                Some(bone) => tf.translation + bone_offsets.offset(vrm_entity, bone)?,
                None => tf.translation,
            };
            let viewport = coordinate.to_viewport_by_world(world_pos)?;
            let offset = offset.unwrap_or(IVec2::ZERO);
            Some(WindowPosition::At(viewport.as_ivec2() + offset))
        }
        _ => None,
    }
}

fn insert_tracking(commands: &mut Commands, webview_entity: Entity, options: &WebviewOpenOptions) {
    if let Some(WebviewOpenPosition::Vrm {
        tracking: Some(true),
        vrm: Some(vrm),
        bone,
        offset,
    }) = &options.position
    {
        let vrm_entity = Entity::from_bits(*vrm);
        commands.entity(webview_entity).insert(WebviewTracking {
            vrm: vrm_entity,
            bone: bone.clone(),
            offset: *offset,
        });
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
