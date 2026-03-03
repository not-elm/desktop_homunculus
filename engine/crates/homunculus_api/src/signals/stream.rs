use crate::prelude::ApiResult;
use crate::signals::{SignalsApi, SignalsChannels};
use async_broadcast::Receiver;
use bevy::prelude::*;
use bevy::tasks::futures_lite::Stream;
use bevy::tasks::futures_lite::stream::unfold;
use bevy_flurx::prelude::*;

impl SignalsApi {
    pub async fn stream(
        self,
        signal: impl Into<String>,
    ) -> ApiResult<impl Stream<Item = serde_json::Value>> {
        let signal = signal.into();
        self.0
            .schedule(move |task| async move {
                let rx = task
                    .will(Update, once::run(obtain_receiver).with(signal))
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
    In(signal): In<String>,
    mut channels: ResMut<SignalsChannels>,
) -> Receiver<serde_json::Value> {
    channels.channel(signal).1.new_receiver()
}
