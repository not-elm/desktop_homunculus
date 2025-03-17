use crate::mascot::Mascot;
use crate::system_param::GlobalScreenPos;
use bevy::ecs::system::SystemParam;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Entity, Query, Without};
use bevy::render::view::RenderLayers;
use bevy::window::Monitor;

#[derive(SystemParam)]
pub struct Monitors<'w, 's> {
    pub monitors: Query<'w, 's, (Entity, &'static Monitor, &'static RenderLayers), Without<Mascot>>,
}

impl Monitors<'_, '_> {
    pub fn find_monitor_from_name(
        &self,
        monitor_name: &str,
    ) -> Option<(Entity, &Monitor, &RenderLayers)> {
        self.monitors.iter().find(|(_, monitor, _)| {
            monitor
                .name
                .as_ref()
                .is_some_and(|name| name == monitor_name)
        })
    }

    pub fn find_monitor_from_global_screen_pos(
        &self,
        global: GlobalScreenPos,
    ) -> Option<(Entity, &Monitor, &RenderLayers)> {
        self.monitors
            .iter()
            .find(|(_, monitor, _)| monitor_rect(monitor).contains(*global))
    }

    pub fn scale_factor(&self, render_layers: &RenderLayers) -> Option<f32> {
        let (_, monitor) = self.find_monitor(render_layers)?;
        Some(monitor.scale_factor as f32)
    }

    #[inline]
    pub fn monitor_pos(&self, render_layers: &RenderLayers) -> Option<Vec2> {
        let (_, monitor, _) = self
            .monitors
            .iter()
            .find(|(_, _, monitor_layers)| monitor_layers == &render_layers)?;
        Some(monitor.physical_position.as_vec2())
    }

    #[inline]
    pub fn find_monitor(&self, render_layers: &RenderLayers) -> Option<(Entity, &Monitor)> {
        self.monitors.iter().find_map(|(entity, monitor, layer)| {
            render_layers.intersects(layer).then_some((entity, monitor))
        })
    }
}

#[inline]
pub fn monitor_rect(monitor: &Monitor) -> Rect {
    let s = monitor.scale_factor as f32;
    let pos = monitor.physical_position.as_vec2() / s;
    let size = monitor.physical_size().as_vec2() / s;
    Rect::from_corners(pos, pos + size)
}
