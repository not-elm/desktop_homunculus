use crate::api;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::{AssetId, AssetResolver};
use homunculus_effects::prelude::{RequestStampEffect, StampOptions};
use std::path::PathBuf;

api!(
    /// Provides effects API to play stamps, etc.
    EffectsApi
);

impl EffectsApi {
    pub async fn stamp(&self, asset_id: AssetId, options: Option<StampOptions>) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(stamp).with((asset_id, options)))
                    .await
            })
            .await?
    }
}

fn stamp(
    In((asset_id, options)): In<(AssetId, Option<StampOptions>)>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
) -> ApiResult {
    let entry = match asset_resolver.resolve(&asset_id) {
        Ok(e) => e,
        Err(_) => {
            warn!("Asset not found: {asset_id}");
            return Ok(());
        }
    };
    let path = format!("asset://{}/{}", entry.mod_name, entry.path.display());
    commands.trigger(RequestStampEffect {
        image_path: PathBuf::from(path),
        options,
    });
    Ok(())
}
