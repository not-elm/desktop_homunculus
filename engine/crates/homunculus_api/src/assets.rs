use crate::api;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{AssetId, AssetRegistry, AssetType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AssetFilter {
    #[serde(rename = "type")]
    pub asset_type: Option<AssetType>,
    #[serde(rename = "mod")]
    pub mod_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AssetInfo {
    pub id: AssetId,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
    #[serde(rename = "mod")]
    pub mod_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

api!(
    /// Provides asset listing API.
    AssetsApi
);

impl AssetsApi {
    pub async fn list(&self, filter: AssetFilter) -> ApiResult<Vec<AssetInfo>> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(list_assets).with(filter)).await
            })
            .await
    }
}

fn list_assets(In(filter): In<AssetFilter>, registry: Res<AssetRegistry>) -> Vec<AssetInfo> {
    registry
        .all()
        .filter(|e| {
            filter
                .asset_type
                .as_ref()
                .is_none_or(|t| t == &e.asset_type)
                && filter.mod_name.as_ref().is_none_or(|m| m == &e.mod_name)
        })
        .map(|e| AssetInfo {
            id: e.id.clone(),
            asset_type: e.asset_type.clone(),
            mod_name: e.mod_name.clone(),
            description: e.description.clone(),
        })
        .collect()
}
