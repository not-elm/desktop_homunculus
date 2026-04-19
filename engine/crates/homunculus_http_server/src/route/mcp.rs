//! HTTP routes for MCP extension registry — register/deregister mod MCP servers.

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use utoipa::ToSchema;

use homunculus_mcp::downstream::{RegisterArgs, RegistrationInfo, SharedMcpExtensionRegistry};

/// Body for `POST /mcp/register`.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub mod_name: String,
    pub mod_slug: String,
    pub mcp_url: String,
}

/// Body for `POST /mcp/deregister`.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterRequest {
    pub mod_slug: String,
}

/// Register a mod's MCP server endpoint.
#[utoipa::path(
    post,
    path = "/mcp/register",
    request_body = RegisterRequest,
    responses((status = 200, description = "registered")),
    tag = "mcp",
)]
pub async fn register(
    State(registry): State<SharedMcpExtensionRegistry>,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    let args = RegisterArgs {
        mod_slug: body.mod_slug,
        mod_name: body.mod_name,
        mcp_url: body.mcp_url,
    };
    let mut reg = registry.0.write().await;
    match reg.add(args).await {
        Ok(()) => StatusCode::OK.into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// Deregister a mod's MCP server.
#[utoipa::path(
    post,
    path = "/mcp/deregister",
    request_body = DeregisterRequest,
    responses((status = 200, description = "deregistered")),
    tag = "mcp",
)]
pub async fn deregister(
    State(registry): State<SharedMcpExtensionRegistry>,
    Json(body): Json<DeregisterRequest>,
) -> impl IntoResponse {
    registry.0.write().await.remove(&body.mod_slug).await;
    StatusCode::OK.into_response()
}

/// List currently registered mod MCP servers (debugging).
#[utoipa::path(
    get,
    path = "/mcp/registrations",
    responses((status = 200, description = "list of registrations", body = Vec<RegistrationInfo>)),
    tag = "mcp",
)]
pub async fn list_registrations(
    State(registry): State<SharedMcpExtensionRegistry>,
) -> Json<Vec<RegistrationInfo>> {
    Json(registry.0.read().await.list_registrations().await)
}
