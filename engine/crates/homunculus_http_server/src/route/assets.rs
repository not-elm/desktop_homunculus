use axum::Json;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use homunculus_api::assets::{AssetFilter, AssetInfo, AssetsApi, ImportAsset, ImportAssetResponse};
use homunculus_api::prelude::ApiError;
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};
use serde::Deserialize;

/// List available assets, optionally filtered by type or mod name.
#[utoipa::path(
    get,
    path = "/assets",
    tag = "assets",
    params(
        ("type" = Option<String>, Query, description = "Filter by asset type"),
        ("mod" = Option<String>, Query, description = "Filter by mod name"),
    ),
    responses(
        (status = 200, description = "List of assets", body = Vec<AssetInfo>),
    ),
)]
pub async fn list(
    State(api): State<AssetsApi>,
    Query(filter): Query<AssetFilter>,
) -> HttpResult<Vec<AssetInfo>> {
    api.list(filter).await.into_http_result()
}

/// Import a local file as a managed asset.
///
/// Copies the file to `~/.homunculus/assets/`, registers it in the
/// asset registry, and persists the record in the database.
#[utoipa::path(
    post,
    path = "/assets/import",
    tag = "assets",
    request_body = ImportAsset,
    responses(
        (status = 200, description = "Asset imported successfully", body = ImportAssetResponse),
        (status = 400, description = "Invalid source path or parameters"),
    ),
)]
pub async fn import(
    State(api): State<AssetsApi>,
    Json(body): Json<ImportAsset>,
) -> HttpResult<ImportAssetResponse> {
    api.import(body).await.into_http_result()
}

/// Query parameters for the asset file endpoint.
#[derive(Deserialize)]
pub struct AssetFileQuery {
    pub id: String,
}

/// Get the raw file content of an asset by its ID.
///
/// Returns the file bytes with a `Content-Type` header inferred from
/// the file extension. Unknown extensions fall back to
/// `application/octet-stream`.
#[utoipa::path(
    get,
    path = "/assets/file",
    tag = "assets",
    params(
        ("id" = String, Query, description = "Asset ID (e.g. '@hmcs/elmer:thumbnail')"),
    ),
    responses(
        (status = 200, description = "Raw file content", content_type = "application/octet-stream"),
        (status = 400, description = "Missing id parameter"),
        (status = 404, description = "Asset not found or file missing"),
        (status = 500, description = "IO error"),
    ),
)]
pub async fn get_asset_file(
    State(api): State<AssetsApi>,
    Query(query): Query<AssetFileQuery>,
) -> Result<Response, ApiError> {
    let path = api.get_file_path(query.id).await?;
    read_and_respond(&path).await
}

/// Reads the file at `path` and builds an HTTP response with inferred MIME type.
async fn read_and_respond(path: &std::path::Path) -> Result<Response, ApiError> {
    let bytes = std::fs::read(path).map_err(|e| map_io_error(e, path))?;
    let mime = mime_guess::from_path(path)
        .first_raw()
        .unwrap_or("application/octet-stream");

    Ok((
        [
            (axum::http::header::CONTENT_TYPE, mime),
            (
                axum::http::header::HeaderName::from_static("x-content-type-options"),
                "nosniff",
            ),
            // Force revalidation on every request. Asset IDs are not strictly
            // content-addressed in this codebase — e.g. `vrm:local:{personaId}`
            // and hash-derived thumbnail IDs can map to different file
            // contents over time (local re-imports overwrite the asset file
            // in place). Without `no-cache` the browser shows stale images
            // after re-imports.
            (axum::http::header::CACHE_CONTROL, "no-cache"),
        ],
        bytes,
    )
        .into_response())
}

/// Maps a `std::io::Error` to an `ApiError`.
///
/// `NotFound` maps to `AssetNotFound` (404), all other IO errors map
/// to `InvalidInput` which falls through to the 500 catch-all.
fn map_io_error(err: std::io::Error, path: &std::path::Path) -> ApiError {
    if err.kind() == std::io::ErrorKind::NotFound {
        ApiError::AssetNotFound(path.display().to_string().into())
    } else {
        ApiError::InvalidInput(format!("Failed to read file {}: {err}", path.display()))
    }
}
