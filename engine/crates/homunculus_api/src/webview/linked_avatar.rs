use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_core::prelude::{AvatarRegistry, LinkedAvatar};

impl WebviewApi {
    /// Gets the linked avatar ID for a webview.
    pub async fn linked_avatar(&self, webview: Entity) -> ApiResult<Option<String>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_avatar).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked avatar for a webview by avatar ID.
    pub async fn set_linked_avatar(&self, webview: Entity, avatar_id: String) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_linked_avatar).with((webview, avatar_id)),
                )
                .await
            })
            .await?
    }

    /// Removes the linked avatar from a webview.
    pub async fn unlink_avatar(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(unlink_avatar).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked avatar by resolving a VRM entity to its avatar ID.
    ///
    /// Used by the deprecated `PUT /webviews/{entity}/linked-vrm` route.
    /// Looks up the entity in `AvatarRegistry` to find the avatar ID.
    pub async fn set_linked_avatar_by_entity(
        &self,
        webview: Entity,
        vrm_entity: Entity,
    ) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_linked_avatar_by_entity).with((webview, vrm_entity)),
                )
                .await
            })
            .await?
    }

    /// Gets the linked avatar as an entity (resolves via AvatarRegistry).
    ///
    /// Returns the entity for the linked avatar, or `None` if not linked
    /// or the avatar is not registered.
    pub async fn linked_avatar_entity(&self, webview: Entity) -> ApiResult<Option<Entity>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_avatar_entity).with(webview))
                    .await
            })
            .await?
    }
}

fn get_linked_avatar(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedAvatar>, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<Option<String>> {
    match webviews.get(webview) {
        Ok(linked) => Ok(linked.map(|l| l.0.clone())),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

fn set_linked_avatar(
    In((webview, avatar_id)): In<(Entity, String)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands
            .entity(webview)
            .try_insert(LinkedAvatar(avatar_id));
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn unlink_avatar(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.entity(webview).remove::<LinkedAvatar>();
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}

fn set_linked_avatar_by_entity(
    In((webview, vrm_entity)): In<(Entity, Entity)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
    registry: Res<AvatarRegistry>,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    let avatar_id = registry
        .get_id(vrm_entity)
        .ok_or_else(|| ApiError::EntityNotFound)?;
    commands
        .entity(webview)
        .try_insert(LinkedAvatar(avatar_id.to_string()));
    Ok(())
}

fn get_linked_avatar_entity(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedAvatar>, With<bevy_cef::prelude::WebviewSource>>,
    registry: Res<AvatarRegistry>,
) -> ApiResult<Option<Entity>> {
    match webviews.get(webview) {
        Ok(Some(linked)) => {
            let id = homunculus_core::avatar::AvatarId::new(&linked.0)
                .map_err(|e| ApiError::InvalidAvatarId(e.to_string()))?;
            Ok(registry.get(&id))
        }
        Ok(None) => Ok(None),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}
