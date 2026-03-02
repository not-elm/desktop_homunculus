//! `/display` provides the methods for display control.

use axum::Json;
use homunculus_api::prelude::axum::HttpResult;
use homunculus_screen::prelude::GlobalDisplays;

/// Get all display information.
#[utoipa::path(
    get,
    path = "/",
    tag = "displays",
    responses(
        (status = 200, description = "List of all displays", body = GlobalDisplays),
    ),
)]
pub async fn all() -> HttpResult<GlobalDisplays> {
    Ok(Json(GlobalDisplays::find_all()))
}
