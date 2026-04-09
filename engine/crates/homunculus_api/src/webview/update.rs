use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    TransformConstraint, WebviewMeshSize, WebviewOffset, WebviewPatchRequest,
};
use homunculus_effects::{Entity, Update};

impl WebviewApi {
    /// Updates the offset of a webview entity.
    pub async fn set_offset(&self, webview: Entity, offset: WebviewOffset) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_offset).with((webview, offset)))
                    .await
            })
            .await?
    }

    /// Replaces the mesh size of a webview entity.
    pub async fn set_size(&self, webview: Entity, size: Vec2) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_size).with((webview, size)))
                    .await
            })
            .await?
    }

    /// Updates the viewport size of a webview entity.
    /// bevy_cef automatically resizes the CEF browser when `WebviewSize` changes.
    pub async fn set_viewport_size(&self, webview: Entity, size: Vec2) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_viewport_size).with((webview, size)))
                    .await
            })
            .await?
    }

    /// Applies a partial update to a webview entity atomically.
    pub async fn patch(&self, webview: Entity, request: WebviewPatchRequest) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(patch).with((webview, request)))
                    .await
            })
            .await?
    }
}

fn set_offset(
    In((webview, offset)): In<(Entity, WebviewOffset)>,
    mut commands: Commands,
    webviews: Query<(Entity, Option<&TransformConstraint>), With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    let Ok((_, existing)) = webviews.get(webview) else {
        return Err(ApiError::WebviewNotFound(webview));
    };
    let base = existing.copied().unwrap_or_default();
    commands.entity(webview).try_insert(TransformConstraint {
        intended_offset: offset.0,
        ..base
    });
    Ok(())
}

fn set_size(
    In((webview, size)): In<(Entity, Vec2)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        let new_mesh = meshes.add(Plane3d::new(Vec3::Z, size));
        commands
            .entity(webview)
            .try_insert((Mesh3d(new_mesh), WebviewMeshSize(size)));
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn set_viewport_size(
    In((webview, size)): In<(Entity, Vec2)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.entity(webview).try_insert(WebviewSize(size));
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn patch(
    In((webview, request)): In<(Entity, WebviewPatchRequest)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    webviews: Query<
        (Entity, Option<&TransformConstraint>),
        With<bevy_cef::prelude::WebviewSource>,
    >,
) -> ApiResult<()> {
    let Ok((_, existing_constraint)) = webviews.get(webview) else {
        return Err(ApiError::WebviewNotFound(webview));
    };

    let mut entity_commands = commands.entity(webview);

    if let Some(offset) = request.offset {
        let base = existing_constraint.copied().unwrap_or_default();
        entity_commands.try_insert(TransformConstraint {
            intended_offset: offset.0,
            ..base
        });
    }

    if let Some(constraints) = request.constraints {
        let base = existing_constraint.copied().unwrap_or_default();
        entity_commands.try_insert(TransformConstraint {
            rotation_follow: constraints.rotation_follow.unwrap_or(base.rotation_follow),
            max_tilt_degrees: constraints
                .max_tilt_degrees
                .unwrap_or(base.max_tilt_degrees),
            lock_scale: constraints.lock_scale.unwrap_or(base.lock_scale),
            intended_offset: base.intended_offset,
        });
    }

    if let Some(size) = request.size {
        let new_mesh = meshes.add(Plane3d::new(Vec3::Z, size));
        entity_commands.try_insert((Mesh3d(new_mesh), WebviewMeshSize(size)));
    }

    if let Some(viewport_size) = request.viewport_size {
        entity_commands.try_insert(WebviewSize(viewport_size));
    }

    Ok(())
}
