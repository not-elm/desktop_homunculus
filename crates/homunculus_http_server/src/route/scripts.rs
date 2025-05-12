//! `/scripts` provides methods for executing scripts.

use crate::route::ModuleSourceRequest;
use axum::Json;
use axum::extract::State;
use homunculus_api::prelude::ScriptsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

/// Execute a JavaScript file.
///
/// ### Path
///
/// `POST /scripts/js`
pub async fn js(
    State(api): State<ScriptsApi>,
    Json(body): Json<ModuleSourceRequest>,
) -> HttpResult {
    api.call_javascript(body.source).await.into_http_result()
}
