mod list;
mod send;
mod stream;

use async_broadcast::{Receiver, Sender};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api;

/// Information about an active signal channel.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct SignalInfo {
    /// The signal channel name.
    pub signal: String,
    /// The number of active subscribers.
    pub subscribers: usize,
}

#[derive(Resource, Debug, Clone, Deref, DerefMut, Default)]
pub(crate) struct SignalsChannels(
    HashMap<String, (Sender<serde_json::Value>, Receiver<serde_json::Value>)>,
);

api!(
    /// Provides access to the signals API.
    ///
    /// Signals are used to send messages to the application or external processes.
    SignalsApi
);

pub(super) struct SignalsApiPlugin;

impl Plugin for SignalsApiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SignalsChannels>();
    }
}

impl SignalsChannels {
    pub fn send_blocking(
        &mut self,
        signal: impl Into<String>,
        value: serde_json::Value,
    ) -> Result<(), async_broadcast::SendError<serde_json::Value>> {
        let (sender, _) = self.channel(signal);
        sender.broadcast_blocking(value)?;
        Ok(())
    }

    pub fn channel(
        &mut self,
        signal: impl Into<String>,
    ) -> &mut (Sender<serde_json::Value>, Receiver<serde_json::Value>) {
        self.0.entry(signal.into()).or_insert_with(|| {
            let (mut sender, receiver) = async_broadcast::broadcast(100);
            sender.set_overflow(true);
            (sender, receiver)
        })
    }
}
