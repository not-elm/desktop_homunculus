//! RPC tool implementations for the MCP handler.

use super::super::HomunculusMcpHandler;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use serde::{Deserialize, Serialize};
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

        // Read-lock the registry to look up mod registration.
        let (port, timeout_ms) = {
            let reg = self.rpc_registry.read().unwrap_or_else(|e| e.into_inner());

            let registration = match reg.get(&args.mod_name) {
                Some(r) => r,
                None => {
                    return format!(
                        "Error: MOD '{}' is not registered. Use 'homunculus://rpc' resource to list available mods.",
                        args.mod_name
                    );
                }
            };

            let method_meta = match registration.methods.get(&args.method) {
                Some(m) => m,
                None => {
                    return format!(
                        "Error: Method '{}' not found on MOD '{}'. Available methods: {}",
                        args.method,
                        args.mod_name,
                        registration
                            .methods
                            .keys()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
            };

            let timeout_ms = method_meta.timeout.unwrap_or(DEFAULT_RPC_TIMEOUT_MS);
            (registration.port, timeout_ms)
        };

        let url = format!("http://127.0.0.1:{}/{}", port, args.method);
        let timeout = Duration::from_millis(timeout_ms);

        let client = reqwest::Client::new();
        let mut request = client.post(&url).timeout(timeout);

        if let Some(body) = &args.body {
            request = request.json(body);
        } else {
            request = request.header("content-length", "0");
        }

        match request.send().await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                Err(e) => format!("Error: Failed to read response body: {e}"),
            },
            Err(e) => {
                if e.is_timeout() {
                    format!(
                        "Error: RPC call to '{}/{}' timed out after {timeout_ms}ms",
                        args.mod_name, args.method
                    )
                } else {
                    format!(
                        "Error: RPC call to '{}/{}' failed: {e}",
                        args.mod_name, args.method
                    )
                }
            }
        }
    }
}
