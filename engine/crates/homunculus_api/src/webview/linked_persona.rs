use crate::error::{ApiError, ApiResult};
use crate::prelude::WebviewApi;
use bevy::prelude::*;
use bevy_flurx::action::once;
use homunculus_core::prelude::{LinkedPersona, PersonaId};

impl WebviewApi {
    /// Gets the linked persona ID for a webview.
    pub async fn linked_persona(&self, webview: Entity) -> ApiResult<Option<PersonaId>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_linked_persona).with(webview))
                    .await
            })
            .await?
    }

    /// Sets the linked persona for a webview.
    pub async fn set_linked_persona(
        &self,
        webview: Entity,
        persona_id: PersonaId,
    ) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(set_linked_persona).with((webview, persona_id)),
                )
                .await
            })
            .await?
    }

    /// Removes the linked persona from a webview.
    pub async fn unlink_persona(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(unlink_persona).with(webview))
                    .await
            })
            .await?
    }
}

fn get_linked_persona(
    In(webview): In<Entity>,
    webviews: Query<Option<&LinkedPersona>, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<Option<PersonaId>> {
    match webviews.get(webview) {
        Ok(linked) => Ok(linked.map(|l| l.0.clone())),
        Err(_) => Err(ApiError::WebviewNotFound(webview)),
    }
}

fn set_linked_persona(
    In((webview, persona_id)): In<(Entity, PersonaId)>,
    mut commands: Commands,
    webviews: Query<Entity, With<bevy_cef::prelude::WebviewSource>>,
) -> ApiResult<()> {
    if !webviews.contains(webview) {
        return Err(ApiError::WebviewNotFound(webview));
    }
    commands
        .entity(webview)
        .try_insert(LinkedPersona(persona_id));
    Ok(())
}

fn unlink_persona(
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
