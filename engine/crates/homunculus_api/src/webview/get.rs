use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    LinkedPersona, TransformArgs, TransformConstraint, WebviewConstraints, WebviewInfo,
    WebviewMeshSize,
};

impl WebviewApi {
    pub async fn get(&self, webview: Entity) -> ApiResult<WebviewInfo> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_webview).with(webview))
                    .await
            })
            .await?
    }
}

fn get_webview(
    In(webview): In<Entity>,
    webviews: Query<(
        &OriginalWebviewSource,
        Option<&WebviewMeshSize>,
        &WebviewSize,
        Option<&TransformConstraint>,
        Option<&LinkedPersona>,
    )>,
) -> ApiResult<WebviewInfo> {
    match webviews.get(webview) {
        Ok((source, mesh_size, viewport_size, constraint, linked_persona)) => {
            let c = constraint.copied().unwrap_or_default();
            Ok(WebviewInfo {
                entity: webview,
                source: webview_source_to_info(&source.0, true),
                size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                viewport_size: viewport_size.0,
                transform: TransformArgs {
                    translation: Some(c.intended_offset),
                    rotation: None,
                    scale: None,
                },
                constraints: WebviewConstraints {
                    rotation_follow: Some(c.rotation_follow),
                    max_tilt_degrees: Some(c.max_tilt_degrees),
                    lock_scale: Some(c.lock_scale),
                },
                linked_persona: linked_persona.map(|l| l.0.clone()),
            })
        }
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}
