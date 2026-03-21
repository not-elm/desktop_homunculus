use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    CharacterRegistry, LinkedCharacter, WebviewInfo, WebviewMeshSize, WebviewOffset,
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
        Option<&LinkedCharacter>,
    )>,
    registry: Res<CharacterRegistry>,
) -> ApiResult<WebviewInfo> {
    match webviews.get(webview) {
        Ok((source, mesh_size, viewport_size, offset, linked_character)) => {
            let (character_id, vrm_entity_bits) = resolve_linked(linked_character, &registry);
            Ok(WebviewInfo {
                entity: webview,
                source: webview_source_to_info(&source.0, true),
                size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                viewport_size: viewport_size.0,
                offset: *offset,
                linked_character: character_id,
                linked_vrm: vrm_entity_bits,
            })
        }
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

/// Derives both `linked_character` and deprecated `linked_vrm` from the component.
fn resolve_linked(
    linked: Option<&LinkedCharacter>,
    registry: &CharacterRegistry,
) -> (Option<String>, Option<u64>) {
    let Some(linked) = linked else {
        return (None, None);
    };
    let character_id = linked.0.clone();
    let entity_bits = homunculus_core::character::CharacterId::new(&character_id)
        .ok()
        .and_then(|id| registry.get(&id))
        .map(|e| e.to_bits());
    (Some(character_id), entity_bits)
}
