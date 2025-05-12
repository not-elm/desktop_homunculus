use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::GlobalNamespace;
use bevy_mod_scripting::script_bindings;

pub(super) struct GlobalScriptsPlugin;

impl Plugin for GlobalScriptsPlugin {
    fn build(&self, app: &mut App) {
        register_global_functions(app.world_mut());
    }
}

#[script_bindings(name = "global_functions", unregistered, remote)]
#[allow(unused)]
impl GlobalNamespace {
    pub fn info(message: String) {
        info!("{}", message);
    }

    pub fn warn(message: String) {
        warn!("{}", message);
    }

    pub fn error(message: String) {
        error!("{}", message);
    }
}
