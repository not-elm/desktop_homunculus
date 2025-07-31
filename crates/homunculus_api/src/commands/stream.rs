use crate::commands::{CommandsApi, CommandsChannels};
use crate::prelude::ApiResult;
use async_broadcast::Receiver;
use bevy::prelude::*;
use bevy::tasks::futures_lite::Stream;
use bevy::tasks::futures_lite::stream::unfold;
use bevy_flurx::prelude::*;

impl CommandsApi {
    pub async fn stream(
        self,
        command: impl Into<String>,
    ) -> ApiResult<impl Stream<Item = serde_json::Value>> {
        let command = command.into();
        self.0
            .schedule(move |task| async move {
                let rx = task
                    .will(Update, once::run(obtain_receiver).with(command))
                    .await;
                unfold(rx.clone(), |mut rx| async move {
                    let v = rx.recv().await.ok()?;
                    Some((v, rx))
                })
            })
            .await
    }
}

fn obtain_receiver(
    In(command): In<String>,
    mut channels: ResMut<CommandsChannels>,
) -> Receiver<serde_json::Value> {
    channels.channel(command).1.new_receiver()
}
