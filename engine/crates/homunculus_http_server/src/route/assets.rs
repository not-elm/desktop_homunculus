use axum::extract::{Query, State};
use homunculus_api::assets::{AssetFilter, AssetInfo, AssetsApi};
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
