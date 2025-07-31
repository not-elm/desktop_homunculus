use crate::commands::CommandsChannels;
use crate::prelude::{ApiResult, CommandsApi};
use bevy::prelude::*;
use bevy_flurx::prelude::*;

impl CommandsApi {
    pub async fn send(
        self,
        command: impl Into<String>,
        payload: serde_json::Value,
    ) -> ApiResult<()> {
        let command = command.into();
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(send_command).with((command, payload)))
                    .await
            })
            .await?
    }
}

fn send_command(
    In((command, payload)): In<(String, serde_json::Value)>,
    mut channels: ResMut<CommandsChannels>,
) -> ApiResult {
    channels.send_blocking(command, payload)?;
    Ok(())
}
