use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_cef::prelude::{PreloadScripts, WebviewExtendStandardMaterial, WebviewSize};
use bevy_flurx::action::once;
use bevy_vrm1::prelude::{Cameras, HeadBoneEntity};
use homunculus_core::prelude::{
    AssetResolver, AssetType, LinkedVrm, WebviewMeshSize, WebviewOffset, WebviewOpenOptions,
    WebviewSource, WebviewSourceInfo,
};
use homunculus_effects::{Entity, Update};

/// Tracks the original API-level source used to create/navigate a webview.
#[derive(Component, Debug, Clone)]
pub(crate) struct OriginalWebviewSource(pub WebviewSource);

impl WebviewApi {
    /// Opens a global webview in world space.
    pub async fn open(&self, options: WebviewOpenOptions) -> ApiResult<Entity> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(create_global_webview).with(options))
                    .await
            })
            .await?
    }
}

pub(crate) struct WebviewOpenPlugin;

impl Plugin for WebviewOpenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (visible, track_for_linked_vrm));
    }
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

#[allow(clippy::too_many_arguments)]
fn create_global_webview(
    In(options): In<WebviewOpenOptions>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WebviewExtendStandardMaterial>>,
    cameras: Cameras,
    asset_resolver: AssetResolver,
) -> ApiResult<Entity> {
    let webview_source = source_to_webview_source(&options.source, &asset_resolver)?;

    let webview = spawn_webview_entity(
        &mut commands,
        &mut meshes,
        &mut materials,
        &cameras,
        webview_source,
        &options,
    );

    commands
        .entity(webview)
        .try_insert(OriginalWebviewSource(options.source.clone()));
    insert_preload_scripts(&mut commands, webview);

    if let Some(vrm) = options.linked_vrm {
        commands.entity(webview).try_insert(LinkedVrm(vrm));
    }
    Ok(webview)
}

/// Converts our API-level `WebviewSource` to bevy_cef's `WebviewSource` component.
pub(crate) fn source_to_webview_source(
    source: &WebviewSource,
    asset_resolver: &AssetResolver,
) -> ApiResult<bevy_cef::prelude::WebviewSource> {
    match source {
        WebviewSource::Html { content } => Ok(bevy_cef::prelude::WebviewSource::inline(content)),
        WebviewSource::Url { url } => Ok(bevy_cef::prelude::WebviewSource::new(url)),
        WebviewSource::Local { id } => {
            let entry = asset_resolver
                .resolve(id)
                .map_err(|_| ApiError::AssetNotFound(id.clone()))?;
            if entry.asset_type != AssetType::Html {
                return Err(ApiError::AssetTypeMismatch {
                    id: id.clone(),
                    expected: AssetType::Html,
                    actual: entry.asset_type.clone(),
                });
            }
            // Use forward slashes for URL compatibility. Windows backslash paths
            // in `cef://localhost/C:\...` URLs can be mangled by CEF's URL parser.
            let path = entry.absolute_path.display().to_string().replace('\\', "/");
            Ok(bevy_cef::prelude::WebviewSource::local(path))
        }
    }
}

/// Converts our API-level `WebviewSource` to `WebviewSourceInfo` for responses.
pub(crate) fn webview_source_to_info(
    source: &WebviewSource,
    include_content: bool,
) -> WebviewSourceInfo {
    match source {
        WebviewSource::Url { url } => WebviewSourceInfo::Url { url: url.clone() },
        WebviewSource::Html { content } => WebviewSourceInfo::Html {
            content: if include_content {
                Some(content.clone())
            } else {
                None
            },
        },
        WebviewSource::Local { id } => WebviewSourceInfo::Local { id: id.clone() },
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_webview_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<WebviewExtendStandardMaterial>>,
    cameras: &Cameras,
    webview_source: bevy_cef::prelude::WebviewSource,
    options: &WebviewOpenOptions,
) -> Entity {
    let size = options.size.unwrap_or(Vec2::splat(0.7));
    let mut entity_commands = commands.spawn((
        Name::new("Webview"),
        webview_source,
        cameras.all_layers(),
        NotShadowCaster,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, size))),
        MeshMaterial3d(materials.add(WebviewExtendStandardMaterial {
            base: StandardMaterial {
                unlit: true,
                alpha_mode: AlphaMode::Premultiplied,
                ..default()
            },
            ..default()
        })),
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        Visibility::Hidden,
        Transform::default(),
        options.offset.unwrap_or_default(),
        WebviewMeshSize(size),
    ));

    if let Some(size) = options.viewport_size {
        entity_commands.try_insert(WebviewSize(size));
    }

    entity_commands.id()
}

fn track_for_linked_vrm(
    par_commands: ParallelCommands,
    head_bones: Query<&HeadBoneEntity>,
    global_transforms: Query<&GlobalTransform>,
    webviews: Query<(Entity, &LinkedVrm, &WebviewOffset, &Transform)>,
) {
    webviews
        .par_iter()
        .for_each(|(entity, linked_vrm, offset, tf)| {
            let Ok(head_bone) = head_bones.get(linked_vrm.0) else {
                return;
            };
            let Ok(p) = global_transforms.get(head_bone.0) else {
                return;
            };
            let mut new_tf = *tf;
            new_tf.translation = p.translation() + offset.0;
            par_commands.command_scope(|mut commands| {
                commands.entity(entity).try_insert(new_tf);
            });
        });
}

fn insert_preload_scripts(commands: &mut Commands, webview: Entity) {
    commands
        .entity(webview)
        .try_insert(PreloadScripts::from([&include_str!(
            "../webview/webviewEntity.js"
        )
        .replace("undefined", &webview.to_bits().to_string())]));
}
