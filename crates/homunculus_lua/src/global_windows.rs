use crate::ScriptVal;
use bevy::prelude::*;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::GlobalScreenPos;
use homunculus_screen::{GlobalWindow, GlobalWindows};

pub(super) struct GlobalWindowsScriptPlugin;

impl Plugin for GlobalWindowsScriptPlugin {
    fn build(&self, app: &mut App) {
        register_functions(app.world_mut());
        register_global_screen_pos_functions(app.world_mut());
    }
}

#[script_bindings(remote, name = "global_screen_pos_functions")]
impl GlobalScreenPos {
    pub fn new(x: f32, y: f32) -> ScriptVal<GlobalScreenPos> {
        ScriptVal::new(GlobalScreenPos(Vec2::new(x, y)))
    }
}

#[script_bindings(remote)]
impl GlobalWindows {
    pub fn find_all() -> ScriptVal<Vec<GlobalWindow>> {
        let Some(windows) = GlobalWindows::find_all() else {
            return ScriptVal::new(vec![]);
        };
        ScriptVal::new(windows.0)
    }

    pub fn find_sitting_window(x: f32, y: f32) -> Option<ScriptVal<GlobalWindow>> {
        let drop_pos = GlobalScreenPos(Vec2::new(x, y));
        let windows = GlobalWindows::find_all()?;
        Some(ScriptVal::new(windows.find_sitting_window(drop_pos)?))
    }
}
