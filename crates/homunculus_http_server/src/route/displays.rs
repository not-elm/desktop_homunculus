//! `/display` provides the methods for display control.

use axum::Json;
use homunculus_api::prelude::axum::HttpResult;
use homunculus_screen::prelude::GlobalDisplays;

/// Get all display information.
///
/// ### Path
///
/// `GET /displays/all`
pub async fn all() -> HttpResult<GlobalDisplays> {
    Ok(Json(GlobalDisplays::find_all()))
}
