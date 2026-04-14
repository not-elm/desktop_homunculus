//! VRM tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use bevy::math::Vec2;
use homunculus_api::entities::MoveTarget;
use homunculus_api::persona::CreatePersona;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for the `spawn_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpawnCharacterParams {
    /// Persona ID for the new character (URL-safe: [a-zA-Z0-9_-], max 64 chars).
    pub id: String,
    /// Asset ID of the VRM model to attach (e.g. "vrm:my-model").
    pub asset: String,
    /// Optional display name for the character.
    pub name: Option<String>,
    /// Optional persona profile text describing the character.
    pub profile: Option<String>,
    /// Optional free-text personality description for agent prompts.
    pub personality: Option<String>,
    /// Optional viewport x position to place the character.
    pub x: Option<f32>,
    /// Optional viewport y position to place the character.
    pub y: Option<f32>,
}

/// Parameters for the `select_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectCharacterParams {
    /// Display name (or persona ID) of the character to select.
    pub name: String,
}

/// Parameters for the `remove_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacterParams {
    /// Display name (or persona ID) of the character to remove.
    /// If omitted, removes the active character.
    pub name: Option<String>,
}

/// Parameters for the `set_expression` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetExpressionParams {
    /// Map of expression names to weight values (0.0-1.0).
    pub expressions: Option<HashMap<String, f32>>,
    /// Mode: "modify" (default, partial update), "set" (replace all), or "clear" (reset).
    pub mode: Option<String>,
}

/// Parameters for the `set_persona` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetPersonaParams {
    /// The persona profile text.
    pub profile: Option<String>,
    /// Free-text personality description for agent prompts.
    #[serde(default)]
    pub personality: Option<String>,
}

/// Parameters for the `set_look_at` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SetLookAtParams {
    /// Look-at mode: "cursor" to follow the mouse, or "none" to disable.
    pub mode: String,
}

#[rmcp::tool_router(router = vrm_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Get the current state of all desktop characters.
    #[tool(
        name = "get_character_snapshot",
        description = "Get the current state of all desktop characters including name, persona ID, position, active expressions, playing animations, persona, and lookAt state.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn get_character_snapshot(&self) -> String {
        match self.vrm_api.snapshot().await {
            Ok(snapshots) => {
                // Auto-select the first character if none is active.
                if !snapshots.is_empty() && self.active_persona_id().is_none() {
                    self.set_active_character(Some(snapshots[0].persona.id.clone()));
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
        description = "Spawn a new VRM character on the desktop. Creates a persona and attaches a VRM model. Use the homunculus://assets resource to discover available VRM model assets. Returns the persona ID.",
        annotations(destructive_hint = false, open_world_hint = false)
    )]
    async fn spawn_character(&self, params: Parameters<SpawnCharacterParams>) -> String {
        let args = params.0;

        let create_args = CreatePersona {
            id: args.id.clone(),
            name: args.name.clone(),
            profile: args.profile,
            personality: args.personality,
            ..Default::default()
        };

        let snap = match self.persona_api.create(create_args).await {
            Ok(p) => p,
            Err(e) => return format!("Error creating persona: {e}"),
        };
        let persona_id = snap.persona.id.clone();

        let result = self
            .persona_api
            .attach_vrm(persona_id.clone(), args.asset.clone())
            .await;

        match result {
            Ok(updated) => {
                let display = updated
                    .persona
                    .name
                    .as_deref()
                    .unwrap_or(updated.persona.id.as_ref());

                // Optionally move to viewport position.
                if let (Some(x), Some(y)) = (args.x, args.y)
                    && let Ok(entity) = self.persona_api.resolve(persona_id.clone()).await
                {
                    let target = MoveTarget::Viewport {
                        position: Vec2::new(x, y),
                    };
                    if let Err(e) = self.entities_api.move_to(entity, target).await {
                        return format!(
                            "Spawned character '{display}' (persona {}) but failed to move: {e}",
                            persona_id
                        );
                    }
                }

                self.set_active_character(Some(persona_id.clone()));
                format!("Spawned character '{display}' (persona {})", persona_id)
            }
            Err(e) => format!("Error attaching VRM to persona '{}': {e}", persona_id),
        }
    }

    /// Switch the active character by name.
    #[tool(
        name = "select_character",
        description = "Switch the active character by display name (or persona ID). All subsequent tools will target this character. Use get_character_snapshot to see available characters.",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn select_character(&self, params: Parameters<SelectCharacterParams>) -> String {
        let name = &params.0.name;
        match self.resolve_persona_by_name(name).await {
            Ok(persona) => {
                let display = persona.name.as_deref().unwrap_or(persona.id.as_ref());
                self.set_active_character(Some(persona.id.clone()));
                format!("Selected character '{display}' (persona {})", persona.id)
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    /// Remove a character (persona) from the desktop.
    #[tool(
        name = "remove_character",
        description = "Remove a character (persona + attached VRM) from the desktop. If no name is given, removes the active character.",
        annotations(idempotent_hint = true, open_world_hint = false)
    )]
    async fn remove_character(&self, params: Parameters<RemoveCharacterParams>) -> String {
        let persona_id = if let Some(name) = &params.0.name {
            match self.resolve_persona_by_name(name).await {
                Ok(p) => p.id,
                Err(e) => return format!("Error: {e}"),
            }
        } else {
            match self.active_persona_id() {
                Some(id) => id,
                None => {
                    return "No active character. Specify a name or use select_character first."
                        .to_string();
                }
            }
        };

        match self.persona_api.delete(persona_id.clone()).await {
            Ok(()) => {
                // Clear active character if it was the removed one.
                if self.active_persona_id().as_ref() == Some(&persona_id) {
                    self.set_active_character(None);
                }
                format!("Removed character (persona {})", persona_id)
            }
            Err(e) => format!("Error removing character: {e}"),
        }
    }

    /// Set facial expression weights on the active character.
    #[tool(
        name = "set_expression",
        description = "Set facial expression weights on the active character. Common expressions: happy, sad, angry, surprised, relaxed, neutral, aa, ih, ou, ee, oh, blink. Weights are 0.0-1.0. Modes: \"modify\" (default, partial update), \"set\" (replace all), \"clear\" (reset to animation control).",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
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

    /// Update the active character's persona profile and personality.
    #[tool(
        name = "set_persona",
        description = "Update the active character's persona profile and/or personality text. This affects how the character is perceived in AI conversations.",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn set_persona(&self, params: Parameters<SetPersonaParams>) -> String {
        let persona_id = match self.active_persona_id() {
            Some(id) => id,
            None => {
                return "No active character. Use select_character or spawn_character first."
                    .to_string();
            }
        };

        let args = params.0;
        let patch = homunculus_api::persona::PatchPersona {
            profile: args.profile,
            personality: args.personality,
            ..Default::default()
        };

        match self.persona_api.patch(persona_id.clone(), patch).await {
            Ok(_) => format!("Updated persona for character (persona {})", persona_id),
            Err(e) => format!("Error setting persona: {e}"),
        }
    }

    /// Control where the active character looks.
    #[tool(
        name = "set_look_at",
        description = "Control where the active character looks. Use \"cursor\" to follow the mouse cursor, or \"none\" to disable look-at (character looks forward).",
        annotations(
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
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
