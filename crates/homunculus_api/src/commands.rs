use async_broadcast::{Receiver, Sender};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

mod send;
mod stream;

use crate::api;

api!(
    /// Provides access to the commands API.
    ///
    /// Commands are used to send messages to the application or external processes.
    CommandsApi
);

#[derive(Resource, Debug, Clone, Deref, DerefMut, Default)]
struct CommandsChannels(HashMap<String, (Sender<serde_json::Value>, Receiver<serde_json::Value>)>);

impl CommandsChannels {
    pub fn send_blocking(
        &mut self,
        command: impl Into<String>,
        value: serde_json::Value,
    ) -> Result<(), async_broadcast::SendError<serde_json::Value>> {
        let (sender, _) = self.channel(command);
        sender.broadcast_blocking(value)?;
        Ok(())
    }

    pub fn channel(
        &mut self,
        command: impl Into<String>,
    ) -> &mut (Sender<serde_json::Value>, Receiver<serde_json::Value>) {
        self.0.entry(command.into()).or_insert_with(|| {
            let (mut sender, receiver) = async_broadcast::broadcast(100);
            sender.set_overflow(true);
            (sender, receiver)
        })
    }
}

pub(super) struct CommandsApiPlugin;

impl Plugin for CommandsApiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandsChannels>();
    }
}
