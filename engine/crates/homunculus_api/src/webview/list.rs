use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    AvatarRegistry, LinkedAvatar, WebviewInfo, WebviewMeshSize, WebviewOffset,
};

impl WebviewApi {
    /// Lists all open webviews.
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
        &WebviewOffset,
        Option<&LinkedAvatar>,
    )>,
    registry: Res<AvatarRegistry>,
) -> Vec<WebviewInfo> {
    webviews
        .iter()
        .map(
            |(entity, source, mesh_size, viewport_size, offset, linked_avatar)| {
                let (avatar_id, vrm_entity_bits) =
                    resolve_linked_fields(linked_avatar, &registry);
                WebviewInfo {
                    entity,
                    source: webview_source_to_info(&source.0, false),
                    size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                    viewport_size: viewport_size.0,
                    offset: *offset,
                    linked_avatar: avatar_id,
                    linked_vrm: vrm_entity_bits,
                }
            },
        )
        .collect()
}

/// Derives both `linked_avatar` and deprecated `linked_vrm` from the component.
fn resolve_linked_fields(
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
