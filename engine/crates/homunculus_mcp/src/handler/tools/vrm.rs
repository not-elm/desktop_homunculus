//! VRM tool implementations for the MCP handler.
//!
//! These tools operate on VRM characters and provide backward compatibility
//! with the pre-character entity-based API.

use super::super::HomunculusMcpHandler;
use bevy::math::Vec2;
use homunculus_api::entities::MoveTarget;
use homunculus_core::prelude::{CharacterId, Persona};
use homunculus_utils::schema::asset::AssetId;
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

/// Parameters for the `select_character_by_name` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectCharacterByNameParams {
    /// Name of the character to select.
    pub name: String,
}

/// Parameters for the `remove_character_by_name` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacterByNameParams {
    /// Name of the character to remove. If omitted, removes the active character.
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

#[rmcp::tool_router(router = vrm_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Get the current state of all desktop characters.
    #[tool(
        name = "get_character_snapshot",
        description = "Get the current state of all desktop characters including name, entity ID, position, active expressions, playing animations, persona, and lookAt state."
    )]
    async fn get_character_snapshot(&self) -> String {
        match self.vrm_api.snapshot().await {
            Ok(snapshots) => match serde_json::to_string_pretty(&snapshots) {
                Ok(json) => json,
                Err(e) => format!("Error serializing snapshots: {e}"),
            },
            Err(e) => format!("Error getting character snapshots: {e}"),
        }
    }

    /// Spawn a new VRM character on the desktop.
    ///
    /// Deprecated: prefer `create_character` + `attach_vrm` for new integrations.
    #[tool(
        name = "spawn_character",
        description = "Spawn a new VRM character on the desktop. Deprecated: use create_character + attach_vrm instead. Use the homunculus://assets resource to discover available VRM model assets. Returns the character ID."
    )]
    async fn spawn_character(&self, params: Parameters<SpawnCharacterParams>) -> String {
        let args = params.0;
        let character_id_str = generate_character_id(&args);

        let id = match CharacterId::new(&character_id_str) {
            Ok(id) => id,
            Err(e) => return format!("Error: {e}"),
        };
        let name = args
            .name
            .clone()
            .unwrap_or_else(|| character_id_str.clone());
        let asset_id = AssetId::new(&args.asset);

        let entity = match self
            .character_api
            .create(id.clone(), asset_id.clone(), name, true)
            .await
        {
            Ok(e) => e,
            Err(e) => return format!("Error creating character: {e}"),
        };

        if let Err(e) = self.character_api.attach_vrm(id.clone(), asset_id).await {
            return format!("Created character '{character_id_str}' but failed to attach VRM: {e}");
        }

        if let Some(persona) = build_persona(&args) {
            let _ = self.character_api.set_persona(id, persona).await;
        }

        if let (Some(x), Some(y)) = (args.x, args.y) {
            let target = MoveTarget::Viewport {
                position: Vec2::new(x, y),
            };
            if let Err(e) = self.entities_api.move_to(entity, target).await {
                return format!("Spawned character '{character_id_str}' but failed to move: {e}");
            }
        }

        self.set_active_character(Some(character_id_str.clone()));
        format!("Spawned character '{character_id_str}'")
    }

    /// Switch the active character by name.
    #[tool(
        name = "select_character_by_name",
        description = "Switch the active character by display name. All subsequent tools will target this character. Use get_character_snapshot to see available characters. Deprecated: prefer select_character with a character ID."
    )]
    async fn select_character_by_name(
        &self,
        params: Parameters<SelectCharacterByNameParams>,
    ) -> String {
        let name = params.0.name;
        match self.vrm_api.find_by_name(name.clone()).await {
            Ok(entity) => {
                let entity_id = entity.to_bits();
                // Try to find the character ID for this entity via the character list.
                if let Ok(characters) = self.character_api.list().await {
                    for info in &characters {
                        if let Ok(id) = CharacterId::new(&info.id)
                            && let Ok(e) = self.character_api.resolve(id).await
                            && e == entity
                        {
                            self.set_active_character(Some(info.id.clone()));
                            return format!(
                                "Selected character '{name}' (character '{}')",
                                info.id
                            );
                        }
                    }
                }
                format!("Selected character '{name}' (entity {entity_id})")
            }
            Err(e) => format!("Error finding character '{name}': {e}"),
        }
    }

    /// Remove a VRM character from the desktop by name.
    #[tool(
        name = "remove_character_by_name",
        description = "Remove a VRM character from the desktop by display name. If no name is given, removes the active character. Deprecated: prefer remove_character with a character ID."
    )]
    async fn remove_character_by_name_tool(
        &self,
        params: Parameters<RemoveCharacterByNameParams>,
    ) -> String {
        if let Some(name) = &params.0.name {
            self.remove_character_by_name(name).await
        } else {
            self.remove_active_character().await
        }
    }

    /// Set facial expression weights on the active character.
    #[tool(
        name = "set_expression",
        description = "Set facial expression weights on the active character. Common expressions: happy, sad, angry, surprised, relaxed, neutral, aa, ih, ou, ee, oh, blink. Weights are 0.0-1.0. Modes: \"modify\" (default, partial update), \"set\" (replace all), \"clear\" (reset to animation control)."
    )]
    async fn set_expression(&self, params: Parameters<SetExpressionParams>) -> String {
        let (_id, entity) = match self.resolve_character_with_vrm().await {
            Ok(v) => v,
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
        let (_id, entity) = match self.resolve_character_with_vrm().await {
            Ok(v) => v,
            Err(e) => return format!("Error: {e}"),
        };

        let args = params.0;
        let persona = Persona {
            profile: args.profile,
            personality: args.personality,
            ..Default::default()
        };

        match self.vrm_api.set_persona(entity, persona).await {
            Ok(()) => "Updated persona.".to_string(),
            Err(e) => format!("Error setting persona: {e}"),
        }
    }

    /// Control where the active character looks.
    #[tool(
        name = "set_look_at",
        description = "Control where the active character looks. Use \"cursor\" to follow the mouse cursor, or \"none\" to disable look-at (character looks forward)."
    )]
    async fn set_look_at(&self, params: Parameters<SetLookAtParams>) -> String {
        let (_id, entity) = match self.resolve_character_with_vrm().await {
            Ok(v) => v,
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

impl HomunculusMcpHandler {
    /// Removes a character by name, looking up its character ID.
    async fn remove_character_by_name(&self, name: &str) -> String {
        let entity = match self.vrm_api.find_by_name(name.to_string()).await {
            Ok(e) => e,
            Err(e) => return format!("Error finding character '{name}': {e}"),
        };

        // Try to destroy via character API by finding its character ID.
        if let Some(id_str) = self.find_character_id_for_entity(entity).await
            && let Ok(id) = CharacterId::new(&id_str)
        {
            match self.character_api.destroy(id).await {
                Ok(()) => {
                    self.clear_active_if_matches(&id_str);
                    return format!("Removed character '{name}' (character '{id_str}')");
                }
                Err(e) => return format!("Error removing character: {e}"),
            }
        }

        // Fallback: despawn via VRM API.
        match self.vrm_api.despawn(entity).await {
            Ok(()) => format!("Removed character '{name}'"),
            Err(e) => format!("Error removing character: {e}"),
        }
    }

    /// Removes the active character/character.
    async fn remove_active_character(&self) -> String {
        match self.resolve_character().await {
            Ok((id, _entity)) => {
                let id_str = id.to_string();
                match self.character_api.destroy(id).await {
                    Ok(()) => {
                        self.set_active_character(None);
                        format!("Removed character '{id_str}'")
                    }
                    Err(e) => format!("Error removing character: {e}"),
                }
            }
            Err(e) => format!("Error: {e}"),
        }
    }
}

/// Generates a character ID from spawn arguments.
///
/// Uses a sanitised version of the name if provided, otherwise derives
/// from the asset ID.
fn generate_character_id(args: &SpawnCharacterParams) -> String {
    let base = args
        .name
        .as_deref()
        .unwrap_or(&args.asset)
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();

    let trimmed = base.trim_matches('-');
    if trimmed.is_empty() {
        "character".to_string()
    } else {
        trimmed[..trimmed.len().min(63)].to_string()
    }
}

/// Builds a [`Persona`] from spawn arguments, if any persona-relevant fields are present.
fn build_persona(args: &SpawnCharacterParams) -> Option<Persona> {
    match (&args.name, &args.persona_profile) {
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
    }
}
