//! Audio tool implementations for the MCP handler.

use homunculus_utils::schema::asset::AssetId;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

use super::super::HomunculusMcpHandler;

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

/// Parameters for the `play_sound` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PlaySoundParams {
    /// Sound effect asset ID (e.g. "se:open").
    pub sound: String,
    /// Volume level from 0.0 to 1.0 (default: 0.8).
    pub volume: Option<f64>,
}

/// Parameters for the `control_bgm` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ControlBgmParams {
    /// Action to perform: "play", "stop", "pause", "resume", or "status".
    pub action: String,
    /// BGM asset ID (required for "play" action).
    pub asset: Option<String>,
    /// Volume level from 0.0 to 1.0 (default: 1.0 for play).
    pub volume: Option<f64>,
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = audio_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Play a sound effect.
    #[tool(
        name = "play_sound",
        description = "Play a sound effect. Use a MOD asset ID (e.g., 'se:open') or a preset name."
    )]
    async fn play_sound(&self, params: Parameters<PlaySoundParams>) -> String {
        let args = params.0;
        let volume = args.volume.unwrap_or(0.8);
        let sound = &args.sound;

        match self
            .audio_se_api
            .play(AssetId::new(sound), volume, 1.0, 0.0)
            .await
        {
            Ok(()) => format!("Playing sound '{sound}'"),
            Err(e) => format!("Error playing sound: {e}"),
        }
    }

    /// Control background music playback.
    #[tool(
        name = "control_bgm",
        description = "Control background music playback. Actions: play (requires asset), stop, pause, resume, status."
    )]
    async fn control_bgm(&self, params: Parameters<ControlBgmParams>) -> String {
        let args = params.0;

        match args.action.as_str() {
            "play" => {
                let Some(asset) = &args.asset else {
                    return "Error: 'asset' is required for the 'play' action.".to_string();
                };
                let volume = args.volume.unwrap_or(1.0);
                match self
                    .audio_bgm_api
                    .play(AssetId::new(asset), true, volume, 1.0, None)
                    .await
                {
                    Ok(()) => format!("Playing BGM '{asset}'"),
                    Err(e) => format!("Error playing BGM: {e}"),
                }
            }
            "stop" => match self.audio_bgm_api.stop(None).await {
                Ok(()) => "Stopped BGM.".to_string(),
                Err(e) => format!("Error stopping BGM: {e}"),
            },
            "pause" => match self.audio_bgm_api.pause().await {
                Ok(()) => "Paused BGM.".to_string(),
                Err(e) => format!("Error pausing BGM: {e}"),
            },
            "resume" => match self.audio_bgm_api.resume().await {
                Ok(()) => "Resumed BGM.".to_string(),
                Err(e) => format!("Error resuming BGM: {e}"),
            },
            "status" => match self.audio_bgm_api.status().await {
                Ok(status) => match serde_json::to_string_pretty(&status) {
                    Ok(json) => json,
                    Err(e) => format!("Error serializing BGM status: {e}"),
                },
                Err(e) => format!("Error getting BGM status: {e}"),
            },
            other => format!(
                "Unknown action '{other}'. Use \"play\", \"stop\", \"pause\", \"resume\", or \"status\"."
            ),
        }
    }
}
