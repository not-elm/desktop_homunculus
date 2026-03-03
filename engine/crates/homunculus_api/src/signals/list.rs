use crate::prelude::{ApiResult, SignalsApi};
use crate::signals::{SignalInfo, SignalsChannels};
use bevy::prelude::*;
use bevy_flurx::prelude::*;

impl SignalsApi {
    /// Returns a list of all active signal channels with their subscriber counts.
    pub async fn list(self) -> ApiResult<Vec<SignalInfo>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_signals)).await })
            .await?
    }
}

fn list_signals(channels: Res<SignalsChannels>) -> ApiResult<Vec<SignalInfo>> {
    let signals = channels
        .iter()
        .map(|(name, (sender, _))| {
            // receiver_count() includes the initial Receiver held in SignalsChannels,
            // so subtract 1 to get the actual SSE subscriber count.
            let subscribers = sender.receiver_count().saturating_sub(1);
            SignalInfo {
                signal: name.clone(),
                subscribers,
            }
        })
        .collect();
    Ok(signals)
}
