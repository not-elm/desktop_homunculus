use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_cef::prelude::{PreloadScripts, WebviewExtendStandardMaterial, WebviewSize};
use bevy_flurx::action::once;
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::{
    AssetResolver, AssetType, LinkedPersona, PersonaIndex, TransformConstraint, WebviewMeshSize,
    WebviewOpenOptions, WebviewSource, WebviewSourceInfo,
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
        app.add_systems(Update, visible);
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
    index: Res<PersonaIndex>,
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

    if let Some(persona_id) = options.linked_persona {
        commands
            .entity(webview)
            .try_insert(LinkedPersona(persona_id.clone()));
        if let Some(persona_entity) = index.get(&persona_id) {
            commands.entity(webview).set_parent_in_place(persona_entity);
        }
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
    let constraint = build_transform_constraint(options);
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
        Transform::from_translation(constraint.intended_offset),
        constraint,
        WebviewMeshSize(size),
    ));

    if let Some(size) = options.viewport_size {
        entity_commands.try_insert(WebviewSize(size));
    }

    entity_commands.id()
}

fn build_transform_constraint(options: &WebviewOpenOptions) -> TransformConstraint {
    let intended_offset = options
        .transform
        .as_ref()
        .and_then(|t| t.translation)
        .unwrap_or(Vec3::new(0.0, 0.0, 10.0));
    let mut constraint = TransformConstraint {
        intended_offset,
        ..Default::default()
    };
    if let Some(c) = &options.constraints {
        if let Some(v) = c.rotation_follow {
            constraint.rotation_follow = v;
        }
        if let Some(v) = c.max_tilt_degrees {
            constraint.max_tilt_degrees = v;
        }
        if let Some(v) = c.lock_scale {
            constraint.lock_scale = v;
        }
    }
    constraint
}

fn insert_preload_scripts(commands: &mut Commands, webview: Entity) {
    commands
        .entity(webview)
        .try_insert(PreloadScripts::from([&include_str!(
            "../webview/webviewEntity.js"
        )
        .replace("undefined", &webview.to_bits().to_string())]));
}
