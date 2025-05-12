use crate::error::ApiResult;
use crate::prelude::WebviewApi;
use crate::webview::ClosingWebviewSounds;
use bevy_flurx::prelude::*;
use homunculus_effects::{AssetServer, AudioPlayer, Commands, Entity, In, Res, ResMut, Update};

impl WebviewApi {
    /// Try closes a webview.
    ///
    /// If it specified a sound to play when closing, it will play that sound before closing the webview.
    pub async fn close(&self, webview: Entity) -> ApiResult<()> {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(close)
                        .with(webview)
                        .then(once::run(play_close_sound).with(webview)),
                )
                .await;
            })
            .await
    }
}

fn close(In(entity): In<Entity>, mut commands: Commands) {
    commands.entity(entity).try_despawn();
}

fn play_close_sound(
    In(webview_entity): In<Entity>,
    mut commands: Commands,
    mut closing_sounds: ResMut<ClosingWebviewSounds>,
    asset_server: Res<AssetServer>,
) {
    if let Some(sound) = closing_sounds.0.remove(&webview_entity) {
        commands.spawn(AudioPlayer::new(asset_server.load(sound.to_string())));
    }
}
