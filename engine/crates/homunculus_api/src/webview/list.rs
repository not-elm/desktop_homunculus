use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::open::{OriginalWebviewSource, webview_source_to_info};
use bevy::prelude::*;
use bevy_cef::prelude::WebviewSize;
use bevy_flurx::action::once;
use homunculus_core::prelude::{
    CharacterRegistry, LinkedCharacter, WebviewInfo, WebviewMeshSize, WebviewOffset,
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
        Option<&LinkedCharacter>,
    )>,
    registry: Res<CharacterRegistry>,
) -> Vec<WebviewInfo> {
    webviews
        .iter()
        .map(
            |(entity, source, mesh_size, viewport_size, offset, linked_character)| {
                let (character_id, vrm_entity_bits) =
                    resolve_linked_fields(linked_character, &registry);
                WebviewInfo {
                    entity,
                    source: webview_source_to_info(&source.0, false),
                    size: mesh_size.map_or(WebviewMeshSize::default().0, |s| s.0),
                    viewport_size: viewport_size.0,
                    offset: *offset,
                    linked_character: character_id,
                    linked_vrm: vrm_entity_bits,
                }
            },
        )
        .collect()
}

/// Derives both `linked_character` and deprecated `linked_vrm` from the component.
fn resolve_linked_fields(
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
