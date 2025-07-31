use crate::ScriptResult;
use crate::util::{json_to_script_value, script_value_to_json};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::core::error::InteropError;
use bevy_mod_scripting::script_bindings;
use homunculus_prefs::PrefsDatabase;

pub(super) struct ScriptPrefsPlugin;

impl Plugin for ScriptPrefsPlugin {
    fn build(&self, app: &mut App) {
        register_prefs_functions(app.world_mut());
    }
}

#[derive(Reflect)]
struct Prefs;

#[script_bindings(name = "prefs_functions")]
#[allow(unused)]
impl Prefs {
    pub fn load(ctx: FunctionCallContext, key: String) -> ScriptResult<Option<ScriptValue>> {
        let value = ctx.world()?.with_global_access(|world| {
            world
                .non_send_resource::<PrefsDatabase>()
                .load(&key)
                .map(json_to_script_value)
        })?;
        Ok(value)
    }

    pub fn save(ctx: FunctionCallContext, key: String, value: ScriptValue) -> ScriptResult<()> {
        ctx.world()?.with_global_access(|world| {
            world
                .non_send_resource::<PrefsDatabase>()
                .save(&key, &script_value_to_json(&value))
                .map_err(|e| InteropError::external_error(Box::new(e)))
        })??;
        Ok(())
    }
}
