use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::GlobalNamespace;
use bevy_mod_scripting::script_bindings;

pub struct RandomScriptPlugin;

impl Plugin for RandomScriptPlugin {
    fn build(&self, app: &mut App) {
        register_random_functions(app.world_mut());
    }
}

#[script_bindings(name = "random_functions", unregistered, remote)]
#[allow(unused)]
impl GlobalNamespace {
    fn random_range(min: i64, max: i64) -> i64 {
        rand::random_range(min..=max)
    }

    fn random_range_float(min: f64, max: f64) -> f64 {
        rand::random_range(min..=max)
    }
}
