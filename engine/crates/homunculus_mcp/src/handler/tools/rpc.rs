//! RPC call helper for the MCP handler.

use std::time::Duration;

/// Default timeout for RPC calls in milliseconds (10 seconds).
pub(crate) const DEFAULT_RPC_TIMEOUT_MS: u64 = 10_000;

/// Sends an HTTP POST to the MOD service's local RPC endpoint.
pub(crate) async fn send_rpc_call(
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
