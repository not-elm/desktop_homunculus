use crate::api;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::*;
use homunculus_core::prelude::{ModMenuMetadata, ModMenuMetadataList};

api!(ModsApi);

impl ModsApi {
    pub async fn fetch_mod_menus(&self) -> ApiResult<Vec<ModMenuMetadata>> {
        self.0
            .schedule(|task| async move { task.will(Update, once::run(fetch_all_mods)).await })
            .await
    }
}

fn fetch_all_mods(menus: Res<ModMenuMetadataList>) -> Vec<ModMenuMetadata> {
    menus.iter().cloned().collect()
}
