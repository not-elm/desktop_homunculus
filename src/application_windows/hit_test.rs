use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Event, Query, Reflect, Trigger};
use bevy::window::Window;

pub struct ApplicationWindowsHitTestPlugin;

impl Plugin for ApplicationWindowsHitTestPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UpdatedHitTest>()
            .add_event::<UpdatedHitTest>()
            .add_observer(update_hit_test);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
pub struct UpdatedHitTest {
    pub window: Entity,
    pub hit_test: bool,
}

fn update_hit_test(trigger: Trigger<UpdatedHitTest>, mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_mut(trigger.window) {
        window.cursor_options.hit_test = trigger.hit_test;
    }
}
