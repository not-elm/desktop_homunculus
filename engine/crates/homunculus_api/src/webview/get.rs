use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    LinkedPersona, PersonaIndex, WebviewInfo, WebviewMeshSize, WebviewOffset,
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
        &WebviewOffset,
        Option<&LinkedPersona>,
    )>,
    index: Res<PersonaIndex>,
) -> ApiResult<WebviewInfo> {
    match webviews.get(webview) {
        Ok((source, mesh_size, viewport_size, offset, linked_persona)) => Ok(WebviewInfo {
            entity: webview,
            source: webview_source_to_info(&source.0, true),
            size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
            viewport_size: viewport_size.0,
            offset: *offset,
            linked_vrm: linked_persona.and_then(|l| index.get(&l.0)),
        }),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}
