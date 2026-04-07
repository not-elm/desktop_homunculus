use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_core::prelude::{LinkedPersona, Persona, PersonaIndex};

impl WebviewApi {
    /// Gets the linked VRM entity for a webview.
    pub async fn linked_vrm(&self, webview: Entity) -> ApiResult<Option<Entity>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_vrm).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked VRM entity for a webview.
    pub async fn set_linked_vrm(&self, webview: Entity, vrm: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_linked_vrm).with((webview, vrm)))
                    .await
            })
            .await?
    }

    /// Removes the linked VRM from a webview.
    pub async fn unlink_vrm(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(unlink_vrm).with(webview)).await
            })
            .await?
    }
}

fn get_linked_vrm(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedPersona>, With<bevy_cef::prelude::WebviewSource>>,
    index: Res<PersonaIndex>,
) -> ApiResult<Option<Entity>> {
    match webviews.get(webview) {
        Ok(linked) => Ok(linked.and_then(|l| index.get(&l.0))),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

fn set_linked_vrm(
    In((webview, vrm)): In<(Entity, Entity)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
    personas: Query<&Persona>,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    let persona = personas
        .get(vrm)
        .map_err(|_| ApiError::EntityNotFound)?;
    commands
        .entity(webview)
        .try_insert(LinkedPersona(persona.id.clone()));
    Ok(())
}

fn unlink_vrm(
    In(webview): In<Entity>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if webviews.contains(webview) {
        commands.entity(webview).remove::<LinkedPersona>();
        Ok(())
    } else {
        Err(ApiError::WebviewNotFound(webview))
    }
}
