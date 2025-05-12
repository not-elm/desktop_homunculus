use crate::vrm::VrmInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;

pub(super) struct VrmRescalePlugin;

impl Plugin for VrmRescalePlugin {
    fn build(&self, app: &mut App) {
        register_vrm_rescale_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_rescale_function")]
#[allow(unused)]
impl VrmInstance {
    fn rescale(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        scale: ScriptVal<Vec3>,
    ) -> ScriptValueResult<VrmInstance> {
        let entity = me.0.entity;
        ctx.world()?.with_global_access(|world| {
            if let Ok(mut tf) = world.query::<&mut Transform>().get_mut(world, entity) {
                tf.scale = scale.into_inner();
            }
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}
