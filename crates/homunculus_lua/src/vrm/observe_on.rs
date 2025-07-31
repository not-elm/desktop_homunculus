use crate::ScriptVal;
use crate::vrm::VrmInstance;
use bevy::prelude::*;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::VrmState;

pub(super) struct VrmObserveOnPlugin;

impl Plugin for VrmObserveOnPlugin {
    fn build(&self, app: &mut App) {
        register_vrm_observe_on_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_observe_on_function")]
#[allow(unused)]
impl VrmInstance {
    fn observe_on(me: ScriptVal<VrmInstance>, state: String) -> ScriptVal<VrmInstance> {
        ScriptVal::new(VrmInstance {
            entity: me.entity,
            observe_on: Some(VrmState(state)),
        })
    }
}
