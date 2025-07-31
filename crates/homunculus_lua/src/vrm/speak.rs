use crate::vrm::VrmInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;

pub(super) struct VrmSpeakPlugin;

impl Plugin for VrmSpeakPlugin {
    fn build(&self, app: &mut App) {
        register_vrm_speak_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_speak_function")]
#[allow(unused)]
impl VrmInstance {
    fn speak(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        _text: String,
        _speaker: u32,
    ) -> ScriptValueResult<VrmInstance> {
        ctx.world()?.with_global_access(|_world| {
            // world.commands().entity(me.entity).trigger(RequestSpeak {
            //     text,
            //     speaker,
            //     subtitle: None,
            //     sender: None,
            // });
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}
