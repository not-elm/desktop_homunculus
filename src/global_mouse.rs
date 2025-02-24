pub mod button;
pub mod cursor;

use crate::global_mouse::button::GlobalMouseButtonPlugin;
use crate::global_mouse::cursor::GlobalMouseCursorPlugin;
use bevy::app::App;
use bevy::math::Vec2;
use bevy::prelude::{Plugin, Resource};
use device_query::{DeviceQuery, DeviceState};

pub struct GlobalMousePlugin;

impl Plugin for GlobalMousePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalMouseHandle>()
            .add_plugins((
                GlobalMouseButtonPlugin,
                GlobalMouseCursorPlugin,
            ));
    }
}

#[derive(Debug, Resource)]
pub struct GlobalMouseHandle(DeviceState);

impl GlobalMouseHandle {
    #[inline]
    fn pressed_left(&self) -> bool {
        self.0.get_mouse().button_pressed[1]
    }

    #[inline]
    fn cursor_pos(&self) -> Option<Vec2> {
        let mouse = self.0.get_mouse();
        Some(Vec2::new(mouse.coords.0 as f32, mouse.coords.1 as f32))
    }
}

impl Default for GlobalMouseHandle {
    fn default() -> Self {
        Self(DeviceState::new())
    }
}




