#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

use crate::global_windows::macos::obtain_homunculus_screen;
use crate::prelude::find_window_from_number;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{App, Plugin, Reflect};
use homunculus_core::prelude::*;

pub mod prelude {
    #[cfg(target_os = "macos")]
    pub use crate::global_windows::macos::*;
    #[cfg(target_os = "windows")]
    pub use crate::global_windows::windows::*;
    pub use crate::global_windows::{GlobalWindow, GlobalWindows};
}

pub(crate) struct HomunculusGlobalWindowsPlugin;

impl Plugin for HomunculusGlobalWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GlobalWindow>()
            .register_type::<GlobalWindows>();
    }
}

#[derive(Debug, Clone, PartialEq, Default, Reflect)]
pub struct GlobalWindow {
    pub title: Option<String>,
    pub frame: Rect,
    #[cfg(target_os = "macos")]
    pub window_layer: i64,
    #[cfg(target_os = "macos")]
    pub window_number: objc2_foundation::NSInteger,
    #[cfg(target_os = "windows")]
    pub hwnd: i64,
}

impl GlobalWindow {
    #[inline]
    pub fn sitting_pos(&self, drop_pos: GlobalViewport) -> GlobalViewport {
        GlobalViewport(Vec2::new(drop_pos.x, self.frame.min.y))
    }

    /// Update the application_windows metadata.
    ///
    /// Returns `true` if the application_windows position has been updated.
    #[inline]
    pub fn update(&self) -> Option<GlobalWindow> {
        #[cfg(target_os = "macos")]
        {
            if let Some(updated) = find_window_from_number(self.window_number) {
                return Some(updated);
            }
        }
        #[cfg(target_os = "windows")]
        {
            if let Some(updated) = update_window(self.hwnd) {
                if updated != self.frame {
                    return Some(GlobalWindow {
                        frame: updated,
                        ..self.clone()
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug, Default, Reflect)]
pub struct GlobalWindows(pub Vec<GlobalWindow>);

impl GlobalWindows {
    pub const fn new(frames: Vec<GlobalWindow>) -> Self {
        Self(frames)
    }

    pub fn find_all() -> Option<Self> {
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        return obtain_homunculus_screen();
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        return None;
    }

    pub fn find_sitting_window(&self, drop_pos: GlobalViewport) -> Option<GlobalWindow> {
        const SITTING_THRESHOLD_HEIGHT: f32 = 80.;
        let mut areas = Vec::new();
        for sitting_area in self.0.iter() {
            if hitting_sitting_area(&sitting_area.frame, *drop_pos, SITTING_THRESHOLD_HEIGHT)
                && !areas
                    .iter()
                    .any(|area: &&GlobalWindow| area.frame.contains(*drop_pos))
            {
                return Some(sitting_area.clone());
            }
            areas.push(sitting_area);
        }
        None
    }
}

fn hitting_sitting_area(
    window_frame: &Rect,
    drop_viewport_point: Vec2,
    threshold_height: f32,
) -> bool {
    let min = window_frame.min;
    let max = window_frame.max;
    let sitting_area = Rect::from_corners(
        Vec2::new(min.x, min.y - threshold_height),
        Vec2::new(max.x, min.y),
    );
    sitting_area.contains(drop_viewport_point)
}

#[cfg(test)]
mod tests {
    use crate::global_windows::{GlobalWindow, GlobalWindows, hitting_sitting_area};
    use bevy::math::{Rect, Vec2};
    use homunculus_core::prelude::GlobalViewport;

    #[test]
    fn test_sitting_area() {
        assert!(hitting_sitting_area(
            &Rect::from_corners(Vec2::new(0., 0.), Vec2::new(100., 100.)),
            Vec2::new(0., 0.),
            40.,
        ));
    }

    #[test]
    fn exclude_intersect_area() {
        let areas = GlobalWindows::new(vec![
            GlobalWindow {
                frame: Rect::from_corners(Vec2::new(0., 0.), Vec2::new(100., 100.)),
                ..Default::default()
            },
            GlobalWindow {
                frame: Rect::from_corners(Vec2::splat(50.), Vec2::splat(100.)),
                ..Default::default()
            },
        ]);
        assert_eq!(
            areas.find_sitting_window(GlobalViewport(Vec2::new(50., 50.))),
            None
        );
    }

    #[test]
    fn intersect_area() {
        let bottom = GlobalWindow {
            frame: Rect::from_corners(Vec2::splat(50.), Vec2::splat(150.)),
            ..Default::default()
        };
        let areas = GlobalWindows::new(vec![
            GlobalWindow {
                frame: Rect::from_corners(Vec2::new(0., 0.), Vec2::new(100., 100.)),
                ..Default::default()
            },
            bottom.clone(),
        ]);
        assert_eq!(
            areas.find_sitting_window(GlobalViewport(Vec2::new(110., 50.))),
            Some(bottom)
        );
    }
}
