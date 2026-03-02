use crate::api;
use crate::error::{ApiError, ApiResult};
use bevy::prelude::*;
use bevy_flurx::prelude::once;
use homunculus_audio::prelude::*;
use homunculus_core::prelude::{AssetId, AssetResolver};

api!(
    /// Provides audio SE API to play one-shot sound effects.
    AudioSeApi
);

api!(
    /// Provides audio BGM API to control background music.
    AudioBgmApi
);

impl AudioSeApi {
    pub async fn play(
        &self,
        asset_id: AssetId,
        volume: f64,
        speed: f64,
        panning: f64,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(play_se).with((asset_id, volume, speed, panning)),
                )
                .await
            })
            .await?
    }
}

impl AudioBgmApi {
    pub async fn play(
        &self,
        asset_id: AssetId,
        is_loop: bool,
        volume: f64,
        speed: f64,
        fade_in: Option<FadeTween>,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(
                    Update,
                    once::run(play_bgm).with((asset_id, is_loop, volume, speed, fade_in)),
                )
                .await
            })
            .await?
    }

    pub async fn stop(&self, fade_out: Option<FadeTween>) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(stop_bgm).with(fade_out)).await
            })
            .await?
    }

    pub async fn pause(&self) -> ApiResult {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(pause_bgm)).await })
            .await?
    }

    pub async fn resume(&self) -> ApiResult {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(resume_bgm)).await })
            .await?
    }

    pub async fn update(
        &self,
        volume: Option<f64>,
        speed: Option<f64>,
        tween: Option<FadeTween>,
    ) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(update_bgm).with((volume, speed, tween)))
                    .await
            })
            .await?
    }

    pub async fn status(&self) -> ApiResult<BgmStatus> {
        self.0
            .schedule(move |task| async move { task.will(Update, once::run(get_bgm_status)).await })
            .await?
    }
}

// --- SE Systems ---

fn play_se(
    In((asset_id, volume, speed, panning)): In<(AssetId, f64, f64, f64)>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
) -> ApiResult {
    let source = match asset_resolver.load::<KiraAudioSource>(&asset_id) {
        Ok(h) => h,
        Err(_) => {
            warn!("Asset not found: {asset_id}");
            return Ok(());
        }
    };
    commands.trigger(RequestSe {
        source,
        volume,
        speed,
        panning,
    });
    Ok(())
}

// --- BGM Systems ---

fn play_bgm(
    In((asset_id, is_loop, volume, speed, fade_in)): In<(
        AssetId,
        bool,
        f64,
        f64,
        Option<FadeTween>,
    )>,
    mut commands: Commands,
    asset_resolver: AssetResolver,
) -> ApiResult {
    let source = match asset_resolver.load::<KiraAudioSource>(&asset_id) {
        Ok(h) => h,
        Err(_) => {
            warn!("Asset not found: {asset_id}");
            return Ok(());
        }
    };
    commands.trigger(RequestBgmPlay {
        source,
        asset_id,
        is_loop,
        volume,
        speed,
        fade_in,
    });
    Ok(())
}

fn stop_bgm(In(fade_out): In<Option<FadeTween>>, mut commands: Commands) -> ApiResult {
    commands.trigger(RequestBgmStop { fade_out });
    Ok(())
}

fn pause_bgm(mut commands: Commands, state: Res<BgmState>) -> ApiResult {
    if state.current_asset.is_none() || state.is_paused {
        return Err(ApiError::BgmNotPlaying);
    }
    commands.trigger(RequestBgmPause);
    Ok(())
}

fn resume_bgm(mut commands: Commands, state: Res<BgmState>) -> ApiResult {
    if !state.is_paused {
        return Err(ApiError::BgmNotPaused);
    }
    commands.trigger(RequestBgmResume);
    Ok(())
}

fn update_bgm(
    In((volume, speed, tween)): In<(Option<f64>, Option<f64>, Option<FadeTween>)>,
    mut commands: Commands,
) -> ApiResult {
    commands.trigger(RequestBgmUpdate {
        volume,
        speed,
        tween,
    });
    Ok(())
}

fn get_bgm_status(state: Res<BgmState>) -> ApiResult<BgmStatus> {
    let playback_state = if state.current_asset.is_none() {
        BgmPlaybackState::Stopped
    } else if state.is_paused {
        BgmPlaybackState::Paused
    } else {
        BgmPlaybackState::Playing
    };

    Ok(BgmStatus {
        asset: state.current_asset.clone(),
        state: playback_state,
        is_loop: state.is_looping,
        volume: state.target_volume,
        speed: state.target_speed,
    })
}
