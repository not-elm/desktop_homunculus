//! Animation tool implementations for the MCP handler.

use std::time::Duration;

use bevy::animation::RepeatAnimation;
use bevy_vrm1::prelude::PlayVrma;
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

/// Parameters for the `play_animation` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayAnimationParams {
    /// VRMA asset ID to play (e.g. "vrma:idle-maid").
    pub asset: String,
    /// Repeat mode: "never" (default), "forever", or a number (e.g. "3").
    pub repeat: Option<String>,
    /// Transition duration in seconds (default: 0.3).
    pub transition_secs: Option<f32>,
    /// Whether to wait for the animation to finish before returning (default: false).
    pub wait: Option<bool>,
    /// Whether to reset spring bones when starting the animation (default: false).
    pub reset_spring_bones: Option<bool>,
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = animation_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Play a VRMA animation on the active character.
    #[tool(
        name = "play_animation",
        description = "Play a VRMA animation on the active character. Use the homunculus://assets resource to discover available VRMA animations."
    )]
    async fn play_animation(&self, params: Parameters<PlayAnimationParams>) -> String {
        let args = params.0;

        let character = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let repeat_str = args.repeat.as_deref().unwrap_or("never");
        let repeat = match repeat_str {
            "never" => RepeatAnimation::Never,
            "forever" => RepeatAnimation::Forever,
            n => match n.parse::<u32>() {
                Ok(count) => RepeatAnimation::Count(count),
                Err(_) => {
                    return format!(
                        "Invalid repeat value '{n}'. Use \"never\", \"forever\", or a number."
                    );
                }
            },
        };

        let transition_secs = args.transition_secs.unwrap_or(0.3).max(0.0);
        if !transition_secs.is_finite() {
            return "Error: transition_secs must be a finite number.".to_string();
        }
        let wait = args.wait.unwrap_or(false);
        let reset_spring_bones = args.reset_spring_bones.unwrap_or(false);

        let asset = &args.asset;
        let vrma_entity = match self.vrm_api.vrma(character, AssetId::new(asset)).await {
            Ok(e) => e,
            Err(e) => return format!("Error getting VRMA entity: {e}"),
        };

        let play_vrma = PlayVrma {
            vrma: vrma_entity,
            repeat,
            transition_duration: Duration::from_secs_f32(transition_secs),
            reset_spring_bones,
        };

        match self.vrma_api.play(play_vrma, wait).await {
            Ok(()) => format!("Playing animation '{asset}'"),
            Err(e) => format!("Error playing animation: {e}"),
        }
    }
}
