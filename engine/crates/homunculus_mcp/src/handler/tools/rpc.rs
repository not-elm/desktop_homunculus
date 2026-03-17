//! RPC tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use homunculus_core::rpc_registry::RpcRegistry;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Default timeout for RPC calls in milliseconds (10 seconds).
const DEFAULT_RPC_TIMEOUT_MS: u64 = 10_000;

/// Parameters for the `call_rpc` tool.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallRpcParams {
    /// The MOD name to call (e.g. "voicevox").
    pub mod_name: String,
    /// The RPC method name to invoke.
    pub method: String,
    /// Optional JSON body to send with the request.
    pub body: Option<serde_json::Value>,
}

#[rmcp::tool_router(router = rpc_tool_router, vis = "pub(super)")]
impl HomunculusMcpHandler {
    /// Call an RPC method on a running MOD service.
    #[tool(
        name = "call_rpc",
        description = "Call a stateful MOD service RPC method. Use 'homunculus://rpc' resource to discover available mods and methods."
    )]
    async fn call_rpc(&self, params: Parameters<CallRpcParams>) -> String {
        let args = params.0;
        let (port, timeout_ms) =
            match resolve_rpc_endpoint(&self.rpc_registry, &args.mod_name, &args.method) {
                Ok(t) => t,
                Err(e) => return e,
            };
        send_rpc_call(
            port,
            &args.mod_name,
            &args.method,
            timeout_ms,
            args.body.as_ref(),
        )
        .await
    }
}

fn resolve_rpc_endpoint(
    rpc_registry: &Arc<RwLock<RpcRegistry>>,
    mod_name: &str,
    method: &str,
) -> Result<(u16, u64), String> {
    let reg = rpc_registry.read().unwrap_or_else(|e| e.into_inner());
    let registration = reg.get(mod_name).ok_or_else(|| {
        format!(
            "Error: MOD '{mod_name}' is not registered. Use 'homunculus://rpc' resource to list available mods."
        )
    })?;
    let method_meta = registration.methods.get(method).ok_or_else(|| {
        let available = registration
            .methods
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Error: Method '{method}' not found on MOD '{mod_name}'. Available methods: {available}"
        )
    })?;
    Ok((
        registration.port,
        method_meta.timeout.unwrap_or(DEFAULT_RPC_TIMEOUT_MS),
    ))
}

async fn send_rpc_call(
    port: u16,
    mod_name: &str,
    method: &str,
    timeout_ms: u64,
    body: Option<&serde_json::Value>,
) -> String {
    let url = format!("http://127.0.0.1:{port}/{method}");
    let client = reqwest::Client::new();
    let mut request = client.post(&url).timeout(Duration::from_millis(timeout_ms));
    if let Some(b) = body {
        request = request.json(b);
    } else {
        request = request.header("content-length", "0");
    }
    match request.send().await {
        Ok(response) => response
            .text()
            .await
            .unwrap_or_else(|e| format!("Error: Failed to read response body: {e}")),
        Err(e) if e.is_timeout() => {
            format!("Error: RPC call to '{mod_name}/{method}' timed out after {timeout_ms}ms")
        }
        Err(e) => format!("Error: RPC call to '{mod_name}/{method}' failed: {e}"),
    }
}
