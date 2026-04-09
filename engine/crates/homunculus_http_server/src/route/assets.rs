use axum::Json;
use axum::extract::{Query, State};
use homunculus_api::assets::{AssetFilter, AssetInfo, AssetsApi, ImportAsset, ImportAssetResponse};
use homunculus_api::prelude::axum::{HttpResult, IntoHttpResult};

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
