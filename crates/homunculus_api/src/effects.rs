use crate::api;
use crate::error::ApiResult;
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_core::prelude::ModModuleSource;
use homunculus_effects::prelude::{RequestSoundEffect, RequestStampEffect, StampOptions};
use std::path::PathBuf;

api!(
    /// Provides effects API to play sounds, stamps, etc.
    EffectsApi
);

impl EffectsApi {
    pub async fn sound(&self, id: ModModuleSource) -> ApiResult {
        self.0
            .schedule(
                move |task| async move { task.will(Update, once::run(play_sound).with(id)).await },
            )
            .await?
    }

    pub async fn stamp(&self, id: ModModuleSource, options: Option<StampOptions>) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(stamp).with((id, options)))
                    .await
            })
            .await?
    }
}

fn play_sound(In(id): In<ModModuleSource>, mut commands: Commands) -> ApiResult {
    commands.trigger(RequestSoundEffect {
        sound_path: PathBuf::from(id.to_string()),
    });
    Ok(())
}

fn stamp(
    In((id, options)): In<(ModModuleSource, Option<StampOptions>)>,
    mut commands: Commands,
) -> ApiResult {
    commands.trigger(RequestStampEffect {
        image_path: PathBuf::from(id.to_string()),
        options,
    });
    Ok(())
}
