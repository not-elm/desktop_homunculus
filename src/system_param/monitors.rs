use crate::mascot::Mascot;
use bevy::ecs::system::SystemParam;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{Entity, Query, With, Without};
use bevy::render::view::RenderLayers;
use bevy::window::{Monitor, PrimaryMonitor};

#[derive(SystemParam)]
pub struct Monitors<'w, 's> {
    pub primary: Query<'w, 's, Entity, With<PrimaryMonitor>>,
    pub monitors: Query<'w, 's, (Entity, &'static Monitor, &'static RenderLayers), Without<Mascot>>,
}

impl Monitors<'_, '_> {
    #[inline]
    pub fn new_render_layers_if_overall_monitor(
        &self,
        current_render_layers: &RenderLayers,
        viewport_pos: Vec2,
    ) -> Option<(Vec2, &RenderLayers)> {
        let (current_monitor_entity, monitor) = self.find_monitor(current_render_layers)?;
        let monitor_pos = monitor.physical_position.as_vec2();
        self
            .monitors
            .iter()
            .filter(|(entity, _, _)| entity != &current_monitor_entity)
            .find_map(|(_, monitor, layers)| {
                if monitor_rect(monitor).contains(viewport_pos + monitor_pos) {
                    Some((viewport_pos + monitor_pos, layers))
                } else {
                    None
                }
            })
    }

    pub fn default_mascot_viewport_pos_and_layers(&self) -> (Vec2, &RenderLayers) {
        let primary_monitor = self.primary.single();
        let monitor = self.monitors.get(primary_monitor).unwrap();
        (monitor.1.physical_position.as_vec2(), monitor.2)
    }

    pub fn find_monitor_from_screen_pos(&self, viewport_pos: Vec2) -> Option<(Entity, &Monitor, &RenderLayers)> {
        self.monitors.iter().find(|(_, monitor, _)| {
            monitor_rect(monitor).contains(viewport_pos)
        })
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
    fn find_monitor(&self, render_layers: &RenderLayers) -> Option<(Entity, &Monitor)> {
        self
            .monitors
            .iter()
            .find_map(|(entity, monitor, monitor_layers)| {
                (monitor_layers == render_layers).then_some((entity, monitor))
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