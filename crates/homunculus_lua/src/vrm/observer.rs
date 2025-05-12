use crate::ScriptVal;
use crate::vrm::VrmInstance;
use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Reflect};
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::VrmState;

pub(super) struct VrmObserverPlugin;

impl Plugin for VrmObserverPlugin {
    fn build(&self, app: &mut App) {
        register_functions(app.world_mut());
        register_vrm_observer_functions(app.world_mut());
    }
}

#[derive(Reflect)]
pub struct VrmObserver {
    pub vrm: Entity,
    pub observe_on: Option<VrmState>,
}

#[script_bindings]
#[allow(unused)]
impl VrmInstance {
    pub fn observer(
        instance: ScriptVal<VrmInstance>,
        observe_on: Option<String>,
    ) -> ScriptVal<VrmObserver> {
        VrmObserver::new(instance, observe_on)
    }
}

#[script_bindings(name = "vrm_observer_functions")]
#[allow(unused)]
impl VrmObserver {
    pub fn new(
        instance: ScriptVal<VrmInstance>,
        observe_on: Option<String>,
    ) -> ScriptVal<VrmObserver> {
        let observe_on = observe_on.map(VrmState);
        ScriptVal::new(VrmObserver {
            vrm: instance.0.entity,
            observe_on,
        })
    }
}
