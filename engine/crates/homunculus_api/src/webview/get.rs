use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    AvatarRegistry, LinkedAvatar, WebviewInfo, WebviewMeshSize, WebviewOffset,
};

impl WebviewApi {
    /// Gets detailed info for a specific webview.
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
        Option<&LinkedAvatar>,
    )>,
    registry: Res<AvatarRegistry>,
) -> ApiResult<WebviewInfo> {
    match webviews.get(webview) {
        Ok((source, mesh_size, viewport_size, offset, linked_avatar)) => {
            let (avatar_id, vrm_entity_bits) = resolve_linked(linked_avatar, &registry);
            Ok(WebviewInfo {
                entity: webview,
                source: webview_source_to_info(&source.0, true),
                size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                viewport_size: viewport_size.0,
                offset: *offset,
                linked_avatar: avatar_id,
                linked_vrm: vrm_entity_bits,
            })
        }
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

/// Derives both `linked_avatar` and deprecated `linked_vrm` from the component.
fn resolve_linked(
    linked: Option<&LinkedAvatar>,
    registry: &AvatarRegistry,
) -> (Option<String>, Option<u64>) {
    let Some(linked) = linked else {
        return (None, None);
    };
    let avatar_id = linked.0.clone();
    let entity_bits = homunculus_core::avatar::AvatarId::new(&avatar_id)
        .ok()
        .and_then(|id| registry.get(&id))
        .map(|e| e.to_bits());
    (Some(avatar_id), entity_bits)
}
