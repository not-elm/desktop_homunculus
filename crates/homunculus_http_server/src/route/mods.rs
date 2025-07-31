//! `/mods` provides methods for interacting with mods.

use axum::extract::State;
use homunculus_api::prelude::ModsApi;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use homunculus_core::prelude::ModMenuMetadata;

/// Fetch all mod menus.
///
/// ### Path
///
/// `GET /mods/menus`
pub async fn menus(State(api): State<ModsApi>) -> HttpResult<Vec<ModMenuMetadata>> {
    api.fetch_mod_menus().await.into_http_result()
}
