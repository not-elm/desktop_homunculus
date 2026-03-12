//! Reaction tool implementations for the MCP handler.

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

/// Parameters for the `play_reaction` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PlayReactionParams {
    /// Reaction name. Available: happy, sad, confused, error, success, thinking, surprised, neutral.
    pub reaction: String,
    /// Optional message to note alongside the reaction.
    pub message: Option<String>,
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = reaction_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Play a named reaction on the desktop character.
    #[tool(
        name = "play_reaction",
        description = "Play a named reaction on the desktop character. The character will change expression and optionally play a sound effect. Available reactions: happy, sad, confused, error, success, thinking, surprised, neutral."
    )]
    async fn play_reaction(&self, params: Parameters<PlayReactionParams>) -> String {
        let args = params.0;
        let reaction = &args.reaction;

        let preset = match super::super::presets::get_preset(reaction) {
            Some(p) => p,
            None => {
                return format!(
                    "Error: Unknown reaction '{reaction}'. Available: {}",
                    super::super::presets::REACTION_NAMES.join(", ")
                );
            }
        };

        let character = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let mut warnings: Vec<String> = Vec::new();

        // Execute expression, VRMA, and SE concurrently.
        let expr_fut = async {
            if preset.expressions.is_empty() {
                self.vrm_api.clear_expressions(character).await
            } else {
                self.vrm_api
                    .modify_expressions(character, preset.expressions)
                    .await
            }
        };

        let vrma_fut = async {
            if let Some(vrma) = preset.vrma {
                let asset_id = AssetId::new(format!("vrma:{vrma}"));
                match self.vrm_api.vrma(character, asset_id).await {
                    Ok(vrma_entity) => {
                        if let Err(e) = self.vrma_api.play(PlayVrma::new(vrma_entity), false).await
                        {
                            return Some(format!("VRMA play error: {e}"));
                        }
                    }
                    Err(e) => {
                        return Some(format!("VRMA lookup error: {e}"));
                    }
                }
            }
            None
        };

        let se_fut = async {
            if let Some(se) = preset.se {
                let asset_id = AssetId::new(format!("se:{se}"));
                if let Err(e) = self.audio_se_api.play(asset_id, 0.8, 1.0, 0.0).await {
                    return Some(format!("SE play error: {e}"));
                }
            }
            None
        };

        let (expr_result, vrma_warning, se_warning) = tokio::join!(expr_fut, vrma_fut, se_fut);

        if let Err(e) = expr_result {
            warnings.push(format!("Expression error: {e}"));
        }
        if let Some(w) = vrma_warning {
            warnings.push(w);
        }
        if let Some(w) = se_warning {
            warnings.push(w);
        }

        let mut result = format!("Played reaction '{reaction}'");
        if let Some(msg) = &args.message {
            result.push_str(&format!(" (message: {msg})"));
        }
        if !warnings.is_empty() {
            result.push_str(&format!("\nWarnings: {}", warnings.join("; ")));
        }
        result
    }
}
