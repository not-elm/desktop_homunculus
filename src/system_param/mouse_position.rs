use crate::global_mouse::cursor::GlobalMouseCursor;
use crate::system_param::monitors::Monitors;
use crate::system_param::GlobalScreenPos;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec2;
use bevy::prelude::Res;
use bevy::render::view::RenderLayers;

#[derive(SystemParam)]
pub struct MousePosition<'w, 's> {
    pub monitors: Monitors<'w, 's>,
    pub cursor: Res<'w, GlobalMouseCursor>,
}

impl MousePosition<'_, '_> {
    #[inline]
    pub fn global(&self) -> GlobalScreenPos {
        self.cursor.global()
    }

    #[inline]
    pub fn local(&self, render_layers: &RenderLayers) -> Option<Vec2> {
        let global_pos = self.cursor.global();
        let monitor_pos = self.monitors.monitor_pos(render_layers)?;
        Some(*global_pos - monitor_pos)
    }
}