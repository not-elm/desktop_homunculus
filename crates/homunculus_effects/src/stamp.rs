pub use bevy::prelude::*;
use bevy::window::{Monitor, PrimaryMonitor};
use bevy_vrm1::prelude::Cameras;
use homunculus_core::prelude::{AppWindows, GlobalViewport, window_local_pos};
use homunculus_screen::prelude::{DisplayId, GlobalDisplays};
use rand::random_range;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

pub(crate) struct StampEffectsPlugin;

impl Plugin for StampEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_stamp)
            .add_observer(apply_request);
    }
}

#[derive(Event, Serialize, Deserialize)]
pub struct RequestStampEffect {
    pub image_path: PathBuf,
    pub options: Option<StampOptions>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StampOptions {
    /// Specify the display to show the effect.
    /// This ID can be obtained from [`GlobalDisplays`](homunculus_screen::prelude::GlobalDisplays).
    /// If not specified, the effect will be displayed on the primary display.
    pub display: Option<DisplayId>,
    /// Specify the bounds of the effect.
    /// The stamp will be randomly placed within this area.
    /// If not specified, the entire display area will be used as the bounds.
    pub bounds: Option<Rect>,
    /// The stamp size.
    /// If not specified, the default image size is 300x300 pixels.
    pub size: Option<Vec2>,
    /// Duration of the stamp effect in seconds.
    /// If not specified, the default duration is 0.8 seconds.
    #[serde(rename = "durationSecs")]
    pub duration_secs: Option<f64>,
}

#[derive(Deref, DerefMut, Component)]
struct StampTimer(pub Timer);

fn despawn_stamp(
    mut commands: Commands,
    mut effects: Query<(Entity, &mut StampTimer)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in effects.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            commands.entity(entity).try_despawn();
        }
    }
}

fn apply_request(
    trigger: Trigger<RequestStampEffect>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: AppWindows,
    cameras: Cameras<Camera2d>,
    primary_window: Query<&Monitor, With<PrimaryMonitor>>,
) {
    let options = trigger.options.as_ref();
    let Some(display_rect) = display_rect(&options, primary_window) else {
        warn!("Stamp effect requires a valid display area.");
        return;
    };
    let stamp_size = options.and_then(|o| o.size).unwrap_or(Vec2::splat(300.0));
    let display_rect = display_bounds(display_rect, &options);
    let bounds = calc_stamp_display_bounds(stamp_size, display_rect);
    let duration = options.and_then(|o| o.duration_secs).unwrap_or(0.8);
    let duration = Duration::from_secs_f64(duration);

    let stamp_viewport = random_image_screen_pos(bounds);
    let Some((window_entity, window, layers)) = windows.find_by_global_viewport(stamp_viewport)
    else {
        return;
    };
    let stamp_pos = cameras
        .to_world_2d_pos_from_viewport(window_entity, window_local_pos(window, stamp_viewport))
        .unwrap_or_default();
    commands.spawn((
        layers.clone(),
        Sprite {
            image: asset_server.load(trigger.image_path.as_path()),
            custom_size: Some(stamp_size),
            ..default()
        },
        Transform::from_translation(stamp_pos.extend(0.0)),
        StampTimer(Timer::new(duration, TimerMode::Once)),
    ));
}

fn display_rect(
    options: &Option<&StampOptions>,
    primary_window: Query<&Monitor, With<PrimaryMonitor>>,
) -> Option<Rect> {
    options
        .and_then(|o| o.display)
        .and_then(|id| {
            let displays = GlobalDisplays::find_all();
            let display = displays.find_by_id(id)?;
            Some(display.frame)
        })
        .or_else(|| {
            let m = primary_window.single().ok()?;
            let p = m.physical_position.as_vec2();
            let size = m.physical_size().as_vec2();
            Some(Rect::from_corners(p, p + size))
        })
}

fn display_bounds(display_rect: Rect, options: &Option<&StampOptions>) -> Rect {
    options
        .and_then(|o| o.bounds)
        .map(|bounds| {
            Rect::from_corners(bounds.min + display_rect.min, bounds.max + display_rect.min)
        })
        .unwrap_or(display_rect)
}

/// Calculates the display area for an image based on the specified screen area and image size.
fn calc_stamp_display_bounds(stamp_size: Vec2, bounds: Rect) -> Rect {
    let limit_size = (bounds.size() - stamp_size).max(Vec2::ZERO);
    Rect::from_center_size(bounds.center(), limit_size)
}

fn random_image_screen_pos(bounds: Rect) -> GlobalViewport {
    let x = random_range(bounds.min.x..=bounds.max.x);
    let y = random_range(bounds.min.y..=bounds.max.y);
    GlobalViewport(Vec2::new(x, y))
}
