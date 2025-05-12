use crate::vrm::VrmInstance;
use crate::{ScriptResult, ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::VrmState;

pub(super) struct VrmSetStatePlugin;

impl Plugin for VrmSetStatePlugin {
    fn build(&self, app: &mut App) {
        register_vrm_set_state_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_set_state_function")]
#[allow(unused)]
impl VrmInstance {
    fn state(ctx: FunctionCallContext, me: ScriptVal<VrmInstance>) -> ScriptResult<String> {
        ctx.world()?.with_global_access(|world| {
            world
                .query::<&VrmState>()
                .get(world, me.entity)
                .map(|s| s.to_string())
                .unwrap_or_default()
        })
    }

    fn set_state(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        state: String,
    ) -> ScriptValueResult<VrmInstance> {
        let entity = me.0.entity;
        ctx.world()?.with_global_access(|world| {
            if let Ok(mut current) = world.query::<&mut VrmState>().get_mut(world, entity) {
                if current.0 != state {
                    *current = VrmState(state.clone());
                }
            }
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}
