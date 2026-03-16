//! RPC registration and proxy endpoints for MOD services.
//!
//! MOD services call `POST /rpc/register` on startup to publish their
//! available methods.  Callers invoke `POST /rpc/call` with `modName`,
//! `method`, and optional `body` in the JSON request body.

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use homunculus_core::rpc_registry::{RpcMethodMeta, RpcRegistration, RpcRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use utoipa::ToSchema;

/// Default per-method proxy timeout (30 s).
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// Body for `POST /rpc/register`.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub mod_name: String,
    pub methods: HashMap<String, RpcMethodMeta>,
}

/// Body for `POST /rpc/deregister`.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterRequest {
    pub mod_name: String,
}

/// Response for `GET /rpc/registrations`.
#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationsResponse {
    pub registrations: HashMap<String, RpcRegistration>,
}

/// Body for `POST /rpc/call`.
#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
    pub mod_name: String,
    pub method: String,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

/// Register or update a MOD service's RPC methods.
///
/// The MOD service calls this endpoint on startup.  The port must have been
/// pre-allocated by the engine; this handler updates the methods map for that
/// port.
#[utoipa::path(
    post,
    path = "/register",
    tag = "rpc",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Methods registered"),
        (status = 404, description = "MOD has no pre-allocated port"),
        (status = 500, description = "Registry lock poisoned"),
    ),
)]
pub async fn register(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Json(body): Json<RegisterRequest>,
) -> Response {
    let mut reg = match write_registry(&registry) {
        Ok(r) => r,
        Err(e) => return e,
    };
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
#[utoipa::path(
    post,
    path = "/deregister",
    tag = "rpc",
    request_body = DeregisterRequest,
    responses(
        (status = 200, description = "MOD deregistered"),
        (status = 500, description = "Registry lock poisoned"),
    ),
)]
pub async fn deregister(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Json(body): Json<DeregisterRequest>,
) -> Response {
    let mut reg = match write_registry(&registry) {
        Ok(r) => r,
        Err(e) => return e,
    };
    reg.deregister(&body.mod_name);
    StatusCode::OK.into_response()
}

/// List all current RPC registrations (for introspection / debugging).
#[utoipa::path(
    get,
    path = "/registrations",
    tag = "rpc",
    responses(
        (status = 200, description = "Current registrations", body = RegistrationsResponse),
        (status = 500, description = "Registry lock poisoned"),
    ),
)]
pub async fn list_registrations(State(registry): State<Arc<RwLock<RpcRegistry>>>) -> Response {
    let reg = match read_registry(&registry) {
        Ok(r) => r,
        Err(e) => return e,
    };
    Json(RegistrationsResponse {
        registrations: reg.all().clone(),
    })
    .into_response()
}

/// Proxy `POST /rpc/call` to the MOD service's local HTTP server.
///
/// The caller specifies `modName` and `method` in the JSON body, along with
/// an optional `body` field that is forwarded to the MOD service.
///
/// Error codes:
/// - `503` — MOD not registered (not yet started or crashed)
/// - `404` — method unknown for this MOD
/// - `504` — MOD service timed out
/// - `502` — MOD service refused the connection
#[utoipa::path(
    post,
    path = "/call",
    tag = "rpc",
    request_body = CallRequest,
    responses(
        (status = 200, description = "RPC method response (JSON)"),
        (status = 404, description = "Method not found"),
        (status = 502, description = "MOD service unreachable"),
        (status = 503, description = "MOD not registered"),
        (status = 504, description = "Timeout exceeded"),
    ),
)]
pub async fn call(
    State(registry): State<Arc<RwLock<RpcRegistry>>>,
    Json(req): Json<CallRequest>,
) -> Response {
    let (port, timeout_ms) = {
        let reg = match read_registry(&registry) {
            Ok(r) => r,
            Err(e) => return e,
        };
        match resolve_proxy_target(&reg, &req.mod_name, &req.method) {
            Ok(t) => t,
            Err(e) => return e,
        }
    };
    forward_to_mod(port, &req.method, &req.mod_name, timeout_ms, req.body).await
}

/// Returns a 500 registry-lock-poisoned error response.
fn lock_poisoned_error() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({"error": "registry lock poisoned"})),
    )
        .into_response()
}

fn write_registry(
    registry: &Arc<RwLock<RpcRegistry>>,
) -> Result<std::sync::RwLockWriteGuard<'_, RpcRegistry>, Response> {
    registry.write().map_err(|_| lock_poisoned_error())
}

fn read_registry(
    registry: &Arc<RwLock<RpcRegistry>>,
) -> Result<std::sync::RwLockReadGuard<'_, RpcRegistry>, Response> {
    registry.read().map_err(|_| lock_poisoned_error())
}

/// Validates mod+method in the registry and returns `(port, timeout_ms)`.
fn resolve_proxy_target(
    reg: &RpcRegistry,
    mod_name: &str,
    method: &str,
) -> Result<(u16, u64), Response> {
    let entry = reg.get(mod_name).ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": format!("Mod '{mod_name}' is not registered")
            })),
        )
            .into_response()
    })?;
    if !entry.methods.is_empty() && !entry.methods.contains_key(method) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Method '{method}' not found in mod '{mod_name}'")
            })),
        )
            .into_response());
    }
    let timeout_ms = entry
        .methods
        .get(method)
        .and_then(|m| m.timeout)
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    Ok((entry.port, timeout_ms))
}

/// Forwards the request body to the MOD service and relays the response.
async fn forward_to_mod(
    port: u16,
    method: &str,
    mod_name: &str,
    timeout_ms: u64,
    body: Option<serde_json::Value>,
) -> Response {
    let url = format!("http://127.0.0.1:{port}/{method}");
    let timeout = std::time::Duration::from_millis(timeout_ms);
    let mut request = reqwest::Client::new().post(&url);
    request = match body {
        Some(value) => request.json(&value),
        None => request
            .header("content-type", "application/json")
            .header("content-length", "0"),
    };
    let result = tokio::time::timeout(timeout, request.send()).await;
    match result {
        Err(_) => (
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
        Ok(Ok(resp)) => relay_mod_response(resp).await,
    }
}

async fn relay_mod_response(resp: reqwest::Response) -> Response {
    let status =
        StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    match resp.bytes().await {
        Ok(bytes) => (
            status,
            axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "application/json")]),
            bytes,
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({
                "error": format!("Failed to read mod response: {e}")
            })),
        )
            .into_response(),
    }
}
