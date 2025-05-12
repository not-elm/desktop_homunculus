use crate::vrma::VrmaInstance;
use crate::vrma::spawn::spawn_vrma;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::script_bindings;
use std::collections::HashMap;

pub(super) struct VrmaChainPlugin;

impl Plugin for VrmaChainPlugin {
    fn build(&self, app: &mut App) {
        register_vrma_chain_functions(app.world_mut());
    }
}

#[script_bindings(name = "vrma_chain_functions")]
#[allow(unused)]
impl VrmaInstance {
    fn chain(
        ctx: FunctionCallContext,
        mut instance: ScriptVal<VrmaInstance>,
        path: String,
        options: Option<HashMap<String, ScriptValue>>,
    ) -> ScriptValueResult<VrmaInstance> {
        let vrm_entity = instance.vrm;
        let (vrma, play) = spawn_vrma(ctx, options, vrm_entity, path)?;
        instance.vrmas.push((vrma, play));
        Ok(instance)
    }
}
