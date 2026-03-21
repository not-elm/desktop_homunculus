//! Character tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use homunculus_core::prelude::CharacterId;
use homunculus_utils::schema::asset::AssetId;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

/// Parameters for the `create_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterParams {
    /// Unique character identifier (lowercase, URL-safe, e.g. "elmer").
    pub id: String,
    /// Asset ID of the VRM model to bind (e.g. "vrm:my-model").
    pub asset_id: String,
    /// Optional display name for the character. Defaults to the ID if omitted.
    pub name: Option<String>,
}

/// Parameters for the `select_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectCharacterParams {
    /// Character ID to select as active.
    pub id: String,
}

/// Parameters for the `remove_character` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCharacterParams {
    /// Character ID to remove. If omitted, removes the active character.
    pub id: Option<String>,
}

/// Parameters for the `attach_vrm` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachVrmParams {
    /// Character ID to attach to. If omitted, uses the active character.
    pub id: Option<String>,
    /// Asset ID of the VRM model to attach (e.g. "vrm:my-model").
    pub asset_id: String,
}

/// Parameters for the `detach_vrm` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DetachVrmParams {
    /// Character ID to detach from. If omitted, uses the active character.
    pub id: Option<String>,
}

#[rmcp::tool_router(router = character_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Create a new character with a bound VRM model asset.
    #[tool(
        name = "create_character",
        description = "Create a new character with a bound VRM model asset. The character ID must be unique, lowercase, and URL-safe. Use homunculus://assets to discover available VRM models. Sets the new character as active."
    )]
    async fn create_character(&self, params: Parameters<CreateCharacterParams>) -> String {
        let args = params.0;

        let id = match CharacterId::new(&args.id) {
            Ok(id) => id,
            Err(e) => return format!("Error: {e}"),
        };
        let name = args.name.unwrap_or_else(|| args.id.clone());
        let asset_id = AssetId::new(&args.asset_id);

        match self.character_api.create(id, asset_id, name, false).await {
            Ok(_entity) => {
                self.set_active_character(Some(args.id.clone()));
                format!("Created character '{}'", args.id)
            }
            Err(e) => format!("Error creating character: {e}"),
        }
    }

    /// List all registered characters.
    #[tool(
        name = "get_characters",
        description = "List all registered characters with their IDs, names, asset IDs, states, and whether a VRM model is currently attached."
    )]
    async fn get_characters(&self) -> String {
        match self.character_api.list().await {
            Ok(characters) => match serde_json::to_string_pretty(&characters) {
                Ok(json) => json,
                Err(e) => format!("Error serializing characters: {e}"),
            },
            Err(e) => format!("Error listing characters: {e}"),
        }
    }

    /// Select a character as the active target for subsequent tools.
    #[tool(
        name = "select_character",
        description = "Select a character as the active target for subsequent tools. Use get_characters to see available character IDs."
    )]
    async fn select_character(&self, params: Parameters<SelectCharacterParams>) -> String {
        let id_str = params.0.id;

        let id = match CharacterId::new(&id_str) {
            Ok(id) => id,
            Err(e) => return format!("Error: {e}"),
        };

        match self.character_api.resolve(id).await {
            Ok(_entity) => {
                self.set_active_character(Some(id_str.clone()));
                format!("Selected character '{id_str}'")
            }
            Err(e) => format!("Error selecting character: {e}"),
        }
    }

    /// Remove a character and its associated VRM model.
    #[tool(
        name = "remove_character",
        description = "Remove a character and its associated VRM model. If no ID is given, removes the active character."
    )]
    async fn remove_character(&self, params: Parameters<RemoveCharacterParams>) -> String {
        let (id, id_str) = if let Some(id_str) = params.0.id {
            match CharacterId::new(&id_str) {
                Ok(id) => (id, id_str),
                Err(e) => return format!("Error: {e}"),
            }
        } else {
            match self.resolve_character().await {
                Ok((id, _entity)) => {
                    let id_str = id.to_string();
                    (id, id_str)
                }
                Err(e) => return format!("Error: {e}"),
            }
        };

        match self.character_api.destroy(id).await {
            Ok(()) => {
                self.clear_active_if_matches(&id_str);
                format!("Removed character '{id_str}'")
            }
            Err(e) => format!("Error removing character: {e}"),
        }
    }

    /// Attach a VRM model to a character.
    #[tool(
        name = "attach_vrm",
        description = "Load and attach a VRM model to a character. If no character ID is given, uses the active character. Use homunculus://assets to discover available VRM models."
    )]
    async fn attach_vrm(&self, params: Parameters<AttachVrmParams>) -> String {
        let args = params.0;
        let (id, id_str) = match self.resolve_character_id(args.id).await {
            Ok(v) => v,
            Err(e) => return e,
        };

        let asset_id = AssetId::new(&args.asset_id);
        match self.character_api.attach_vrm(id, asset_id).await {
            Ok(_entity) => format!("Attached VRM '{}' to character '{id_str}'", args.asset_id),
            Err(e) => format!("Error attaching VRM: {e}"),
        }
    }

    /// Detach the VRM model from a character.
    #[tool(
        name = "detach_vrm",
        description = "Detach the VRM model from a character. The character entity remains but has no visible model. If no character ID is given, uses the active character."
    )]
    async fn detach_vrm(&self, params: Parameters<DetachVrmParams>) -> String {
        let (id, id_str) = match self.resolve_character_id(params.0.id).await {
            Ok(v) => v,
            Err(e) => return e,
        };

        match self.character_api.detach_vrm(id).await {
            Ok(()) => format!("Detached VRM from character '{id_str}'"),
            Err(e) => format!("Error detaching VRM: {e}"),
        }
    }
}

impl HomunculusMcpHandler {
    /// Resolves an optional character ID string, falling back to the active character.
    async fn resolve_character_id(
        &self,
        id: Option<String>,
    ) -> Result<(CharacterId, String), String> {
        if let Some(id_str) = id {
            let id = CharacterId::new(&id_str).map_err(|e| format!("Error: {e}"))?;
            Ok((id, id_str))
        } else {
            let (id, _entity) = self
                .resolve_character()
                .await
                .map_err(|e| format!("Error: {e}"))?;
            let id_str = id.to_string();
            Ok((id, id_str))
        }
    }
}
