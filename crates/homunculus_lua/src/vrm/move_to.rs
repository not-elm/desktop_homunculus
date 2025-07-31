use crate::vrm::VrmInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;

pub(super) struct VrmMoveToPlugin;

impl Plugin for VrmMoveToPlugin {
    fn build(&self, app: &mut App) {
        register_vrm_move_to_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_move_to_function")]
#[allow(unused)]
impl VrmInstance {
    fn move_to(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        dist: ScriptVal<Vec3>,
    ) -> ScriptValueResult<VrmInstance> {
        let entity = me.0.entity;
        let dist = dist.into_inner();
        ctx.world()?.with_global_access(|world| {
            if let Ok(mut tf) = world.query::<&mut Transform>().get_mut(world, entity) {
                tf.translation = dist;
            }
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}
