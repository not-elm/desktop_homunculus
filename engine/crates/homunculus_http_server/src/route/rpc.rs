//! RPC registration and proxy endpoints for MOD services.
//!
//! MOD services call `POST /rpc/register` on startup to publish their
//! available methods.  Callers proxy requests through `POST /rpc/{mod}/{method}`,
//! which forwards the body to the MOD service's local HTTP server.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_core::rpc_registry::{RpcMethodMeta, RpcRegistration, RpcRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Default per-method proxy timeout (30 s).
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// Body for `POST /rpc/register`.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub mod_name: String,
    pub methods: HashMap<String, RpcMethodMeta>,
}

/// Body for `POST /rpc/deregister`.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterRequest {
    pub mod_name: String,
}

/// Response for `GET /rpc/registrations`.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationsResponse {
    pub registrations: HashMap<String, RpcRegistration>,
}

/// Register or update a MOD service's RPC methods.
///
/// The MOD service calls this endpoint on startup.  The port must have been
/// pre-allocated by the engine; this handler updates the methods map for that
/// port.
pub async fn register(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Json(body): Json<RegisterRequest>,
) -> Response {
    let mut reg = match registry.write() {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "registry lock poisoned"})),
            )
                .into_response();
        }
    };

    // The port was pre-allocated when the process was spawned; look it up.
    match reg.get(&body.mod_name).map(|e| e.port) {
        Some(port) => {
            reg.register(body.mod_name, port, body.methods);
            StatusCode::OK.into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Mod '{}' has no pre-allocated port. Was it started by the engine?", body.mod_name)
            })),
        )
            .into_response(),
    }
}

/// Deregister a MOD service's RPC endpoint.
pub async fn deregister(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Json(body): Json<DeregisterRequest>,
) -> Response {
    match registry.write() {
        Ok(mut reg) => {
            reg.deregister(&body.mod_name);
            StatusCode::OK.into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "registry lock poisoned"})),
        )
            .into_response(),
    }
}

/// List all current RPC registrations (for introspection / debugging).
pub async fn list_registrations(State(registry): State<Arc<RwLock<RpcRegistry>>>) -> Response {
    match registry.read() {
        Ok(reg) => {
            let registrations = reg.all().clone();
            Json(RegistrationsResponse { registrations }).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "registry lock poisoned"})),
        )
            .into_response(),
    }
}

/// Proxy `POST /rpc/{mod_name}/{method}` to the MOD service's local HTTP server.
///
/// Error codes:
/// - `503` — MOD not registered (not yet started or crashed)
/// - `404` — method unknown for this MOD
/// - `504` — MOD service timed out
/// - `502` — MOD service refused the connection
pub async fn proxy(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Path((mod_name, method)): Path<(String, String)>,
    body: axum::body::Bytes,
) -> Response {
    let (port, timeout_ms) = {
        let reg = match registry.read() {
            Ok(r) => r,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "registry lock poisoned"})),
                )
                    .into_response();
            }
        };

        let entry = match reg.get(&mod_name) {
            Some(e) => e,
            None => {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(serde_json::json!({
                        "error": format!("Mod '{mod_name}' is not registered")
                    })),
                )
                    .into_response();
            }
        };

        // If the mod has registered methods, verify the requested method exists.
        if !entry.methods.is_empty() && !entry.methods.contains_key(&method) {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Method '{method}' not found in mod '{mod_name}'")
                })),
            )
                .into_response();
        }

        let timeout_ms = entry
            .methods
            .get(&method)
            .and_then(|m| m.timeout)
            .unwrap_or(DEFAULT_TIMEOUT_MS);

        (entry.port, timeout_ms)
    };

    let url = format!("http://127.0.0.1:{port}/{method}");
    let client = reqwest::Client::new();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    let result = tokio::time::timeout(
        timeout,
        client
            .post(&url)
            .header("content-type", "application/json")
            .body(body)
            .send(),
    )
    .await;

    match result {
        Err(_elapsed) => (
            StatusCode::GATEWAY_TIMEOUT,
            Json(serde_json::json!({
                "error": format!("Mod '{mod_name}' method '{method}' timed out after {timeout_ms}ms")
            })),
        )
            .into_response(),
        Ok(Err(e)) if e.is_connect() => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": format!("Mod '{mod_name}' refused connection: {e}")
            })),
        )
            .into_response(),
        Ok(Err(e)) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": format!("Mod '{mod_name}' proxy error: {e}")
            })),
        )
            .into_response(),
        Ok(Ok(resp)) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let bytes = match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    return (
                        StatusCode::BAD_GATEWAY,
                        Json(serde_json::json!({
                            "error": format!("Failed to read mod response: {e}")
                        })),
                    )
                        .into_response();
                }
            };
            (
                status,
                axum::response::AppendHeaders([(
                    axum::http::header::CONTENT_TYPE,
                    "application/json",
                )]),
                bytes,
            )
                .into_response()
        }
    }
}
