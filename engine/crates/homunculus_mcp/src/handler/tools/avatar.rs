//! Avatar tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use homunculus_core::prelude::AvatarId;
use homunculus_utils::schema::asset::AssetId;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};

/// Parameters for the `create_avatar` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAvatarParams {
    /// Unique avatar identifier (lowercase, URL-safe, e.g. "elmer").
    pub id: String,
    /// Asset ID of the VRM model to bind (e.g. "vrm:my-model").
    pub asset_id: String,
    /// Optional display name for the avatar. Defaults to the ID if omitted.
    pub name: Option<String>,
}

/// Parameters for the `select_avatar` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SelectAvatarParams {
    /// Avatar ID to select as active.
    pub id: String,
}

/// Parameters for the `remove_avatar` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAvatarParams {
    /// Avatar ID to remove. If omitted, removes the active avatar.
    pub id: Option<String>,
}

/// Parameters for the `attach_vrm` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AttachVrmParams {
    /// Avatar ID to attach to. If omitted, uses the active avatar.
    pub id: Option<String>,
    /// Asset ID of the VRM model to attach (e.g. "vrm:my-model").
    pub asset_id: String,
}

/// Parameters for the `detach_vrm` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DetachVrmParams {
    /// Avatar ID to detach from. If omitted, uses the active avatar.
    pub id: Option<String>,
}

#[rmcp::tool_router(router = avatar_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Create a new avatar with a bound VRM model asset.
    #[tool(
        name = "create_avatar",
        description = "Create a new avatar with a bound VRM model asset. The avatar ID must be unique, lowercase, and URL-safe. Use homunculus://assets to discover available VRM models. Sets the new avatar as active."
    )]
    async fn create_avatar(&self, params: Parameters<CreateAvatarParams>) -> String {
        let args = params.0;

        let id = match AvatarId::new(&args.id) {
            Ok(id) => id,
            Err(e) => return format!("Error: {e}"),
        };
        let name = args.name.unwrap_or_else(|| args.id.clone());
        let asset_id = AssetId::new(&args.asset_id);

        match self.avatar_api.create(id, asset_id, name, false).await {
            Ok(_entity) => {
                self.set_active_avatar(Some(args.id.clone()));
                format!("Created avatar '{}'", args.id)
            }
            Err(e) => format!("Error creating avatar: {e}"),
        }
    }

    /// List all registered avatars.
    #[tool(
        name = "get_avatars",
        description = "List all registered avatars with their IDs, names, asset IDs, states, and whether a VRM model is currently attached."
    )]
    async fn get_avatars(&self) -> String {
        match self.avatar_api.list().await {
            Ok(avatars) => match serde_json::to_string_pretty(&avatars) {
                Ok(json) => json,
                Err(e) => format!("Error serializing avatars: {e}"),
            },
            Err(e) => format!("Error listing avatars: {e}"),
        }
    }

    /// Select an avatar as the active target for subsequent tools.
    #[tool(
        name = "select_avatar",
        description = "Select an avatar as the active target for subsequent tools. Use get_avatars to see available avatar IDs."
    )]
    async fn select_avatar(&self, params: Parameters<SelectAvatarParams>) -> String {
        let id_str = params.0.id;

        let id = match AvatarId::new(&id_str) {
            Ok(id) => id,
            Err(e) => return format!("Error: {e}"),
        };

        match self.avatar_api.resolve(id).await {
            Ok(_entity) => {
                self.set_active_avatar(Some(id_str.clone()));
                format!("Selected avatar '{id_str}'")
            }
            Err(e) => format!("Error selecting avatar: {e}"),
        }
    }

    /// Remove an avatar and its associated VRM model.
    #[tool(
        name = "remove_avatar",
        description = "Remove an avatar and its associated VRM model. If no ID is given, removes the active avatar."
    )]
    async fn remove_avatar(&self, params: Parameters<RemoveAvatarParams>) -> String {
        let (id, id_str) = if let Some(id_str) = params.0.id {
            match AvatarId::new(&id_str) {
                Ok(id) => (id, id_str),
                Err(e) => return format!("Error: {e}"),
            }
        } else {
            match self.resolve_avatar().await {
                Ok((id, _entity)) => {
                    let id_str = id.to_string();
                    (id, id_str)
                }
                Err(e) => return format!("Error: {e}"),
            }
        };

        match self.avatar_api.destroy(id).await {
            Ok(()) => {
                self.clear_active_if_matches(&id_str);
                format!("Removed avatar '{id_str}'")
            }
            Err(e) => format!("Error removing avatar: {e}"),
        }
    }

    /// Attach a VRM model to an avatar.
    #[tool(
        name = "attach_vrm",
        description = "Load and attach a VRM model to an avatar. If no avatar ID is given, uses the active avatar. Use homunculus://assets to discover available VRM models."
    )]
    async fn attach_vrm(&self, params: Parameters<AttachVrmParams>) -> String {
        let args = params.0;
        let (id, id_str) = match self.resolve_avatar_id(args.id).await {
            Ok(v) => v,
            Err(e) => return e,
        };

        let asset_id = AssetId::new(&args.asset_id);
        match self.avatar_api.attach_vrm(id, asset_id).await {
            Ok(_entity) => format!("Attached VRM '{}' to avatar '{id_str}'", args.asset_id),
            Err(e) => format!("Error attaching VRM: {e}"),
        }
    }

    /// Detach the VRM model from an avatar.
    #[tool(
        name = "detach_vrm",
        description = "Detach the VRM model from an avatar. The avatar entity remains but has no visible model. If no avatar ID is given, uses the active avatar."
    )]
    async fn detach_vrm(&self, params: Parameters<DetachVrmParams>) -> String {
        let (id, id_str) = match self.resolve_avatar_id(params.0.id).await {
            Ok(v) => v,
            Err(e) => return e,
        };

        match self.avatar_api.detach_vrm(id).await {
            Ok(()) => format!("Detached VRM from avatar '{id_str}'"),
            Err(e) => format!("Error detaching VRM: {e}"),
        }
    }
}

impl HomunculusMcpHandler {
    /// Resolves an optional avatar ID string, falling back to the active avatar.
    async fn resolve_avatar_id(&self, id: Option<String>) -> Result<(AvatarId, String), String> {
        if let Some(id_str) = id {
            let id = AvatarId::new(&id_str).map_err(|e| format!("Error: {e}"))?;
            Ok((id, id_str))
        } else {
            let (id, _entity) = self
                .resolve_avatar()
                .await
                .map_err(|e| format!("Error: {e}"))?;
            let id_str = id.to_string();
            Ok((id, id_str))
        }
    }
}
