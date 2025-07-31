use crate::vrm::VrmInstance;
use crate::{ScriptResult, ScriptVal};
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Name, With};
use bevy_mod_scripting::core::bindings::FunctionCallContext;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::vrm::Vrm;

pub(super) struct VrmFindByNamePlugin;

impl Plugin for VrmFindByNamePlugin {
    fn build(&self, app: &mut App) {
        register_vrm_find_by_name_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_find_by_name_function")]
#[allow(unused)]
impl VrmInstance {
    fn find_by_name(
        ctx: FunctionCallContext,
        vrm_name: String,
    ) -> ScriptResult<Option<ScriptVal<VrmInstance>>> {
        let instance = ctx.world()?.with_global_access(|world| {
            world
                .query_filtered::<(Entity, &Name), With<Vrm>>()
                .iter(world)
                .find_map(|(entity, name)| (name.as_str() == vrm_name).then_some(entity))
                .map(|vrm| ScriptVal::new(VrmInstance::new(vrm)))
        })?;
        Ok(instance)
    }
}
