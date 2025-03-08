use crate::global_mouse::GlobalMouseHandle;
use crate::system_param::GlobalScreenPos;
use bevy::app::{App, First, PreStartup};
use bevy::math::Vec2;
use bevy::prelude::{Plugin, Res, ResMut, Resource};

#[derive(Debug, Copy, Clone, PartialEq, Default, Resource)]
pub struct GlobalMouseCursor {
    prev: Vec2,
    current: Vec2,
}

impl GlobalMouseCursor {
    #[inline(always)]
    pub const fn global(&self) -> GlobalScreenPos {
        GlobalScreenPos(self.current)
    }
}

pub struct GlobalMouseCursorPlugin;

impl Plugin for GlobalMouseCursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalMouseCursor>()
            .add_systems(PreStartup, update_cursor)
            .add_systems(First, update_cursor);
    }
}

pub fn update_cursor(
    mut cursor: ResMut<GlobalMouseCursor>,
    global_mouse: Res<GlobalMouseHandle>,
) {
    let Some(pos) = global_mouse.cursor_pos() else {
        return;
    };
    if !cursor.current.abs_diff_eq(pos, 0.1) {
        cursor.prev = cursor.current;
        cursor.current = pos;
    }
}
