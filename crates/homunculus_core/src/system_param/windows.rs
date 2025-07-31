use crate::prelude::{AppWindow, GlobalViewport};
use bevy::ecs::system::SystemParam;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{ContainsEntity, Entity, Pointer, Query, Reflect, Trigger, With};
use bevy::render::camera::NormalizedRenderTarget;
use bevy::render::view::RenderLayers;
use bevy::window::{Window, WindowPosition};
use std::fmt::Debug;

#[derive(SystemParam)]
pub struct AppWindows<'w, 's> {
    pub windows: Query<'w, 's, (Entity, &'static Window, &'static RenderLayers), With<AppWindow>>,
}

impl AppWindows<'_, '_> {
    pub fn screen_rect(&self) -> Rect {
        self.windows
            .iter()
            .fold(Rect::default(), |acc, (_, window, _)| {
                acc.union(window_to_rect(window))
            })
    }

    pub fn global_cursor_pos(&self) -> Option<GlobalViewport> {
        self.windows.iter().find_map(|(_, window, _)| {
            let cursor = window.cursor_position()?;
            let WindowPosition::At(position) = window.position else {
                return None;
            };
            Some(GlobalViewport(position.as_vec2() + cursor))
        })
    }

    pub fn find_by_global_viewport(
        &self,
        pos: GlobalViewport,
    ) -> Option<(Entity, &Window, &RenderLayers)> {
        self.windows
            .iter()
            .find(|(_, window, _)| window_to_rect(window).contains(*pos))
    }

    pub fn find_by_layers(
        &self,
        layers: &RenderLayers,
    ) -> Option<(Entity, &Window, &RenderLayers)> {
        self.windows
            .iter()
            .find(|(_, _, window_layers)| layers.intersects(window_layers))
    }

    pub fn to_global_viewport(&self, window: Entity, local_pos: Vec2) -> Option<GlobalViewport> {
        let (_, window, _) = self.windows.get(window).ok()?;
        let WindowPosition::At(position) = window.position else {
            panic!("Unreachable code");
        };
        Some(GlobalViewport(position.as_vec2() + local_pos))
    }
}

pub fn global_cursor_pos<E: Debug + Clone + Reflect>(
    trigger: &Trigger<Pointer<E>>,
    windows: &AppWindows,
) -> Option<GlobalViewport> {
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return None;
    };
    windows.to_global_viewport(window_ref.entity(), trigger.pointer_location.position)
}

#[inline]
pub fn window_local_pos(window: &Window, global_screen_pos: GlobalViewport) -> Vec2 {
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
