use crate::prelude::{ApiResult, SignalsApi};
use crate::signals::SignalsChannels;
use bevy::prelude::*;
use bevy_flurx::prelude::*;

impl SignalsApi {
    pub async fn send(
        self,
        signal: impl Into<String>,
        payload: serde_json::Value,
    ) -> ApiResult<()> {
        let signal = signal.into();
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(send_signal).with((signal, payload)))
                    .await
            })
            .await?
    }
}

fn send_signal(
    In((signal, payload)): In<(String, serde_json::Value)>,
    mut channels: ResMut<SignalsChannels>,
) -> ApiResult {
    channels.send_blocking(signal, payload)?;
    Ok(())
}
