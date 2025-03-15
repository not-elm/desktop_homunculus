use crate::system_param::GlobalScreenPos;
use bevy::ecs::system::SystemParam;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Entity, Query, With};
use bevy::render::view::RenderLayers;
use bevy::window::{Window, WindowPosition};

#[derive(SystemParam)]
pub struct Windows<'w, 's> {
    pub windows: Query<'w, 's, (Entity, &'static Window, &'static RenderLayers), With<Window>>,
}

impl Windows<'_, '_> {
    pub fn global_cursor_pos(&self) -> Option<GlobalScreenPos> {
        self
            .windows
            .iter()
            .find_map(|(_, window, _)| {
                let cursor = window.cursor_position()?;
                let WindowPosition::At(position) = window.position else {
                    return None;
                };
                Some(GlobalScreenPos(position.as_vec2() + cursor))
            })
    }

    pub fn find_window_from_global_screen_pos(&self, pos: GlobalScreenPos) -> Option<(Entity, &Window, &RenderLayers)> {
        self.windows.iter().find(|(_, window, _)| {
            window_to_rect(window).contains(*pos)
        })
    }

    pub fn to_global_pos(&self, window: Entity, local_pos: Vec2) -> Option<GlobalScreenPos> {
        let (_, window, _) = self.windows.get(window).ok()?;
        let WindowPosition::At(position) = window.position else {
            panic!("Unreachable code");
        };
        Some(GlobalScreenPos(position.as_vec2() + local_pos))
    }
}


#[inline]
pub fn window_local_pos(window: &Window, global_screen_pos: GlobalScreenPos) -> Vec2 {
    let WindowPosition::At(position) = window.position else {
        panic!("Unreachable code");
    };
    *global_screen_pos - position.as_vec2()
}

#[inline]
fn window_to_rect(window: &Window) -> Rect {
    let WindowPosition::At(position) = window.position else {
        panic!("Unreachable code");
    };
    let position = position.as_vec2();
    Rect::from_corners(position, position + window.resolution.size())
}