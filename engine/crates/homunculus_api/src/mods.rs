use crate::api;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{ModInfo, ModMenuMetadata, ModMenuMetadataList, ModRegistry};

api!(
    /// Provides mod listing API.
    ModsApi
);

impl ModsApi {
    pub async fn list(&self) -> ApiResult<Vec<ModInfo>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_mods)).await })
            .await
    }

    pub async fn find_by_name(&self, name: String) -> ApiResult<ModInfo> {
        let n = name.clone();
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(find_mod).with(n)).await },
            )
            .await?
            .ok_or_else(|| ApiError::ModNotFound(name))
    }

    pub async fn menus(&self) -> ApiResult<Vec<ModMenuMetadata>> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(list_menus)).await })
            .await
    }
}

fn list_mods(registry: Res<ModRegistry>) -> Vec<ModInfo> {
    registry.all().to_vec()
}

fn find_mod(In(name): In<String>, registry: Res<ModRegistry>) -> Option<ModInfo> {
    registry.find_by_name(&name).cloned()
}

fn list_menus(menus: Res<ModMenuMetadataList>) -> Vec<ModMenuMetadata> {
    menus.0.clone()
}
