use crate::system_param::cameras::Cameras;
use crate::system_param::monitors::Monitors;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::render::view::RenderLayers;

#[derive(SystemParam)]
pub struct Coordinate<'w, 's> {
    pub cameras: Cameras<'w, 's>,
    pub monitors: Monitors<'w, 's>,
}

impl Coordinate<'_, '_> {
    #[inline]
    pub fn new_render_layers_if_overall_monitor(
        &self,
        current_render_layers: &RenderLayers,
        world_pos: Vec3,
    ) -> Option<(Vec3, &RenderLayers)> {
        let viewport_pos = self.cameras.to_viewport_pos(current_render_layers, world_pos)?;
        let (new_viewport, new_layers) = self.monitors.new_render_layers_if_overall_monitor(current_render_layers, viewport_pos)?;
        Some((self.cameras.to_world_pos(new_layers, new_viewport)?, new_layers))
    }

    /// If the passed position is outside the screen, return the default position and layers of the mascot.
    pub fn initial_mascot_pos_and_layers(&self, load_pos: Vec3) -> (Vec3, RenderLayers) {
        let viewport_pos = self.cameras.to_viewport_pos(&RenderLayers::default(), load_pos).unwrap();
        if let Some((_, _, layers)) = self.monitors.find_monitor_from_screen_pos(viewport_pos) {
            return (load_pos, layers.clone());
        }
        let (viewport_pos, layers) = self.monitors.default_mascot_viewport_pos_and_layers();
        println!("viewport_pos: {:?} {layers:?}", viewport_pos);
        (self.cameras.to_world_pos(layers, viewport_pos).unwrap_or_default(), layers.clone())
    }
}