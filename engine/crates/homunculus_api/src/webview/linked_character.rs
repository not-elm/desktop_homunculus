use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_core::prelude::{CharacterId, CharacterRegistry, LinkedCharacter};

impl WebviewApi {
    /// Gets the linked character ID for a webview.
    pub async fn linked_character(&self, webview: Entity) -> ApiResult<Option<String>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_character).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked character for a webview by character ID.
    pub async fn set_linked_character(
        &self,
        webview: Entity,
        character_id: String,
    ) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_linked_character).with((webview, character_id)),
                )
                .await
            })
            .await?
    }

    /// Removes the linked character from a webview.
    pub async fn unlink_character(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(unlink_character).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked character by resolving a VRM entity to its character ID.
    ///
    /// Used by the deprecated `PUT /webviews/{entity}/linked-vrm` route.
    /// Looks up the entity in `CharacterRegistry` to find the character ID.
    pub async fn set_linked_character_by_entity(
        &self,
        webview: Entity,
        vrm_entity: Entity,
    ) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_linked_character_by_entity).with((webview, vrm_entity)),
                )
                .await
            })
            .await?
    }

    /// Gets the linked character as an entity (resolves via CharacterRegistry).
    ///
    /// Returns the entity for the linked character, or `None` if not linked
    /// or the character is not registered.
    pub async fn linked_character_entity(&self, webview: Entity) -> ApiResult<Option<Entity>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_character_entity).with(webview))
                    .await
            })
            .await?
    }
}

fn get_linked_character(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedCharacter>, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<Option<String>> {
    match webviews.get(webview) {
        Ok(linked) => Ok(linked.map(|l| l.0.to_string())),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

fn set_linked_character(
    In((webview, raw_id)): In<(Entity, String)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    let character_id =
        CharacterId::new(&raw_id).map_err(|e| ApiError::InvalidCharacterId(e.to_string()))?;
    commands
        .entity(webview)
        .try_insert(LinkedCharacter(character_id));
    Ok(())
}

fn unlink_character(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.entity(webview).remove::<LinkedCharacter>();
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn set_linked_character_by_entity(
    In((webview, vrm_entity)): In<(Entity, Entity)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
    registry: Res<CharacterRegistry>,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    let character_id = registry
        .get_id(vrm_entity)
        .ok_or_else(|| ApiError::EntityNotFound)?
        .clone();
    commands
        .entity(webview)
        .try_insert(LinkedCharacter(character_id));
    Ok(())
}

fn get_linked_character_entity(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedCharacter>, With<bevy_cef::prelude::WebviewSource>>,
    registry: Res<CharacterRegistry>,
) -> ApiResult<Option<Entity>> {
    match webviews.get(webview) {
        Ok(Some(linked)) => Ok(registry.get(&linked.0)),
        Ok(None) => Ok(None),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}
