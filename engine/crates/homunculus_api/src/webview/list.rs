use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    LinkedPersona, TransformConstraint, WebviewConstraints, WebviewInfo, WebviewMeshSize,
    WebviewOffset,
};

impl WebviewApi {
    pub async fn list(&self) -> ApiResult<Vec<WebviewInfo>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_webviews)).await })
            .await
    }
}

fn list_webviews(
    webviews: Query<(
        Entity,
        &OriginalWebviewSource,
        Option<&WebviewMeshSize>,
        &WebviewSize,
        Option<&TransformConstraint>,
        Option<&LinkedPersona>,
    )>,
) -> Vec<WebviewInfo> {
    webviews
        .iter()
        .map(
            |(entity, source, mesh_size, viewport_size, constraint, linked_persona)| {
                let c = constraint.copied().unwrap_or_default();
                WebviewInfo {
                    entity,
                    source: webview_source_to_info(&source.0, false),
                    size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                    viewport_size: viewport_size.0,
                    offset: WebviewOffset(c.intended_offset),
                    constraints: WebviewConstraints {
                        rotation_follow: Some(c.rotation_follow),
                        max_tilt_degrees: Some(c.max_tilt_degrees),
                        lock_scale: Some(c.lock_scale),
                    },
                    linked_persona: linked_persona.map(|l| l.0.clone()),
                }
            },
        )
        .collect()
}
