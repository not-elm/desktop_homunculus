//! VRM tool implementations for the MCP handler.

use std::collections::HashMap;

use bevy::math::Vec2;
use homunculus_api::entities::MoveTarget;
use homunculus_api::vrm::VrmSpawnArgs;
use homunculus_core::prelude::Persona;
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

/// Parameters for the `spawn_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpawnCharacterParams {
    /// Asset ID of the VRM model to spawn (e.g. "vrm:my-model").
    pub asset: String,
    /// Optional display name for the character.
    pub name: Option<String>,
    /// Optional persona profile text describing the character.
    pub persona_profile: Option<String>,
    /// Optional viewport x position to place the character.
    pub x: Option<f32>,
    /// Optional viewport y position to place the character.
    pub y: Option<f32>,
}

/// Parameters for the `select_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectCharacterParams {
    /// Name of the character to select.
    pub name: String,
}

/// Parameters for the `remove_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacterParams {
    /// Name of the character to remove. If omitted, removes the active character.
    pub name: Option<String>,
}

/// Parameters for the `set_expression` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetExpressionParams {
    /// Map of expression names to weight values (0.0–1.0).
    pub expressions: Option<HashMap<String, f32>>,
    /// Mode: "modify" (default, partial update), "set" (replace all), or "clear" (reset).
    pub mode: Option<String>,
}

/// Parameters for the `set_persona` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetPersonaParams {
    /// The persona profile text.
    pub profile: String,
    /// Optional personality description.
    pub personality: Option<String>,
}

/// Parameters for the `set_look_at` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetLookAtParams {
    /// Look-at mode: "cursor" to follow the mouse, or "none" to disable.
    pub mode: String,
}

// ---------------------------------------------------------------------------
// Tool implementations
// ---------------------------------------------------------------------------

#[rmcp::tool_router(router = vrm_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Get the current state of all desktop characters.
    #[tool(
        name = "get_character_snapshot",
        description = "Get the current state of all desktop characters including name, entity ID, position, active expressions, playing animations, persona, and lookAt state."
    )]
    async fn get_character_snapshot(&self) -> String {
        match self.vrm_api.snapshot().await {
            Ok(snapshots) => {
                // Auto-select the first character if none is active.
                if !snapshots.is_empty() {
                    let should_set = self
                        .active_character
                        .lock()
                        .ok()
                        .is_some_and(|g| g.is_none());
                    if should_set {
                        let bits = snapshots[0].entity.to_bits();
                        self.set_active_character(Some(bits));
                    }
                }
                match serde_json::to_string_pretty(&snapshots) {
                    Ok(json) => json,
                    Err(e) => format!("Error serializing snapshots: {e}"),
                }
            }
            Err(e) => format!("Error getting character snapshots: {e}"),
        }
    }

    /// Spawn a new VRM character on the desktop.
    #[tool(
        name = "spawn_character",
        description = "Spawn a new VRM character on the desktop. Use the homunculus://assets resource to discover available VRM model assets. Returns the entity ID and name."
    )]
    async fn spawn_character(&self, params: Parameters<SpawnCharacterParams>) -> String {
        let args = params.0;

        let persona = match (&args.name, &args.persona_profile) {
            (Some(name), Some(profile)) => Some(Persona {
                profile: format!("Name: {name}\n{profile}"),
                ..Default::default()
            }),
            (Some(name), None) => Some(Persona {
                profile: format!("Name: {name}"),
                ..Default::default()
            }),
            (None, Some(profile)) => Some(Persona {
                profile: profile.clone(),
                ..Default::default()
            }),
            (None, None) => None,
        };

        let spawn_args = VrmSpawnArgs {
            asset: AssetId::new(&args.asset),
            transform: None,
            persona,
        };

        match self.vrm_api.spawn(spawn_args).await {
            Ok(entity) => {
                let entity_id = entity.to_bits();

                // Optionally move to viewport position.
                if let (Some(x), Some(y)) = (args.x, args.y) {
                    let target = MoveTarget::Viewport {
                        position: Vec2::new(x, y),
                    };
                    if let Err(e) = self.entities_api.move_to(entity, target).await {
                        return format!(
                            "Spawned character (entity {entity_id}) but failed to move: {e}"
                        );
                    }
                }

                self.set_active_character(Some(entity_id));
                format!("Spawned character (entity {entity_id})")
            }
            Err(e) => format!("Error spawning character: {e}"),
        }
    }

    /// Switch the active character by name.
    #[tool(
        name = "select_character",
        description = "Switch the active character by name. All subsequent tools will target this character. Use get_character_snapshot to see available characters."
    )]
    async fn select_character(&self, params: Parameters<SelectCharacterParams>) -> String {
        let name = params.0.name;
        match self.vrm_api.find_by_name(name.clone()).await {
            Ok(entity) => {
                let entity_id = entity.to_bits();
                self.set_active_character(Some(entity_id));
                format!("Selected character '{name}' (entity {entity_id})")
            }
            Err(e) => format!("Error finding character '{name}': {e}"),
        }
    }

    /// Remove a VRM character from the desktop.
    #[tool(
        name = "remove_character",
        description = "Remove a VRM character from the desktop. If no name is given, removes the active character."
    )]
    async fn remove_character(&self, params: Parameters<RemoveCharacterParams>) -> String {
        let entity = if let Some(name) = &params.0.name {
            match self.vrm_api.find_by_name(name.clone()).await {
                Ok(e) => e,
                Err(e) => return format!("Error finding character '{name}': {e}"),
            }
        } else {
            match self.resolve_character().await {
                Ok(e) => e,
                Err(e) => return format!("Error: {e}"),
            }
        };

        let entity_id = entity.to_bits();
        match self.vrm_api.despawn(entity).await {
            Ok(()) => {
                // Clear active character if it was the removed one.
                let is_active = self
                    .active_character
                    .lock()
                    .ok()
                    .is_some_and(|g| *g == Some(entity_id));
                if is_active {
                    self.set_active_character(None);
                }
                format!("Removed character (entity {entity_id})")
            }
            Err(e) => format!("Error removing character: {e}"),
        }
    }

    /// Set facial expression weights on the active character.
    #[tool(
        name = "set_expression",
        description = "Set facial expression weights on the active character. Common expressions: happy, sad, angry, surprised, relaxed, neutral, aa, ih, ou, ee, oh, blink. Weights are 0.0-1.0. Modes: \"modify\" (default, partial update), \"set\" (replace all), \"clear\" (reset to animation control). For preset reactions, use play_reaction instead."
    )]
    async fn set_expression(&self, params: Parameters<SetExpressionParams>) -> String {
        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let args = params.0;
        let mode = args.mode.as_deref().unwrap_or("modify");

        match mode {
            "set" => {
                let expressions = args.expressions.unwrap_or_default();
                match self.vrm_api.set_expressions(entity, expressions).await {
                    Ok(()) => "Expressions set.".to_string(),
                    Err(e) => format!("Error setting expressions: {e}"),
                }
            }
            "modify" => {
                let expressions = args.expressions.unwrap_or_default();
                match self.vrm_api.modify_expressions(entity, expressions).await {
                    Ok(()) => "Expressions modified.".to_string(),
                    Err(e) => format!("Error modifying expressions: {e}"),
                }
            }
            "clear" => match self.vrm_api.clear_expressions(entity).await {
                Ok(()) => "Expressions cleared.".to_string(),
                Err(e) => format!("Error clearing expressions: {e}"),
            },
            other => format!("Unknown mode '{other}'. Use \"set\", \"modify\", or \"clear\"."),
        }
    }

    /// Set the active character's personality profile.
    #[tool(
        name = "set_persona",
        description = "Set the active character's personality profile. This affects how the character is perceived in AI conversations."
    )]
    async fn set_persona(&self, params: Parameters<SetPersonaParams>) -> String {
        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let args = params.0;
        let persona = Persona {
            profile: args.profile,
            personality: args.personality,
            ..Default::default()
        };

        let entity_id = entity.to_bits();
        match self.vrm_api.set_persona(entity, persona).await {
            Ok(()) => format!("Updated persona for character (entity {entity_id})"),
            Err(e) => format!("Error setting persona: {e}"),
        }
    }

    /// Control where the active character looks.
    #[tool(
        name = "set_look_at",
        description = "Control where the active character looks. Use \"cursor\" to follow the mouse cursor, or \"none\" to disable look-at (character looks forward)."
    )]
    async fn set_look_at(&self, params: Parameters<SetLookAtParams>) -> String {
        let entity = match self.resolve_character().await {
            Ok(e) => e,
            Err(e) => return format!("Error: {e}"),
        };

        let mode = &params.0.mode;
        match mode.as_str() {
            "cursor" => match self.vrm_api.look_at_cursor(entity).await {
                Ok(()) => format!("Set look-at mode to '{mode}'"),
                Err(e) => format!("Error setting look-at: {e}"),
            },
            "none" => match self.vrm_api.unlook(entity).await {
                Ok(()) => format!("Set look-at mode to '{mode}'"),
                Err(e) => format!("Error setting look-at: {e}"),
            },
            other => {
                format!("Unknown look-at mode '{other}'. Use \"cursor\" or \"none\".")
            }
        }
    }
}
