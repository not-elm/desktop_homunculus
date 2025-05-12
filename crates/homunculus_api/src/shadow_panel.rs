use crate::api;
use crate::error::{ApiError, ApiResult};
use bevy::app::{Plugin, Startup};
use bevy::asset::Assets;
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{In, IntoScheduleConfigs, NonSend, Query, Res, ResMut, Update};
use bevy_flurx::prelude::once;
use homunculus_prefs::{PrefsDatabase, PrefsKeys};
use homunculus_shadow_panel::{ShadowPanelMaterial, ShadowPanelSetup};

api!(ShadowPanelApi);

impl ShadowPanelApi {
    pub async fn alpha(&self) -> ApiResult<f32> {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(get_shadow_panel_alpha)).await
            })
            .await?
            .ok_or(ApiError::MissingShadowPanel)
    }

    pub async fn set_alpha(&self, alpha: f32) -> ApiResult {
        self.0
            .schedule(move |task| async move {
                task.will(Update, once::run(set_shadow_panel_alpha).with(alpha))
                    .await;
            })
            .await
    }
}

pub struct ShadowPanelApiPlugin;

impl Plugin for ShadowPanelApiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, load_preference.after(ShadowPanelSetup));
    }
}

fn load_preference(
    mut materials: ResMut<Assets<ShadowPanelMaterial>>,
    shadow_panel: Query<&MeshMaterial3d<ShadowPanelMaterial>>,
    preferences: NonSend<PrefsDatabase>,
) {
    if let Some(alpha) = preferences
        .load(PrefsKeys::SHADOW_PANEL_ALPHA)
        .and_then(|v| v.as_f64())
        .map(|v| v as f32)
        && let Ok(handle) = shadow_panel.single()
        && let Some(material) = materials.get_mut(handle)
    {
        material.alpha_factor = alpha;
    };
}

fn get_shadow_panel_alpha(
    materials: Res<Assets<ShadowPanelMaterial>>,
    shadow_panel: Query<&MeshMaterial3d<ShadowPanelMaterial>>,
) -> Option<f32> {
    let handle = shadow_panel.single().ok()?;
    let material = materials.get(handle)?;
    Some(material.alpha_factor)
}

fn set_shadow_panel_alpha(
    In(alpha): In<f32>,
    mut materials: ResMut<Assets<ShadowPanelMaterial>>,
    shadow_panel: Query<&MeshMaterial3d<ShadowPanelMaterial>>,
    prefs: NonSend<PrefsDatabase>,
) {
    let Ok(handle) = shadow_panel.single() else {
        return;
    };
    if let Some(material) = materials.get_mut(handle) {
        material.alpha_factor = alpha;
    }
    let _ = prefs.save(PrefsKeys::SHADOW_PANEL_ALPHA, &alpha);
}
