mod find_by_name;
mod move_to;
mod observe_on;
mod observer;
// mod on_command;
mod on_load;
mod on_range_timer;
mod on_state_enter;
mod on_timer;
mod pointer;
mod render_layers;
mod rescale;
mod spawn;
mod speak;
mod state;

use crate::RunOnMainThread;
use crate::vrm::find_by_name::VrmFindByNamePlugin;
use crate::vrm::move_to::VrmMoveToPlugin;
use crate::vrm::observe_on::VrmObserveOnPlugin;
use crate::vrm::observer::VrmObserverPlugin;
use crate::vrm::on_load::VrmOnLoadPlugin;
use crate::vrm::on_range_timer::VrmOnRangeTimerPlugin;
use crate::vrm::on_state_enter::VrmOnStateEnterPlugin;
use crate::vrm::on_timer::VrmOnTimerPlugin;
use crate::vrm::pointer::VrmPointerPlugin;
use crate::vrm::render_layers::VrmRenderLayersPlugin;
use crate::vrm::rescale::VrmRescalePlugin;
use crate::vrm::spawn::VrmSpawnPlugin;
use crate::vrm::speak::VrmSpeakPlugin;
use crate::vrm::state::VrmSetStatePlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, ScriptValue};
use homunculus_core::prelude::VrmState;

#[doc(hidden)]
pub mod prelude {}

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct VrmHandlers<H>(pub Vec<H>);

impl<H> Default for VrmHandlers<H> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
pub(crate) struct VrmHandlerBase {
    f: DynamicScriptFunction,
    observer_on: Option<VrmState>,
}

impl VrmHandlerBase {
    #[inline]
    pub fn target_state(&self, current: &VrmState) -> bool {
        self.observer_on.as_ref().is_none_or(|s| s == current)
    }
}

impl Default for VrmHandlerBase {
    fn default() -> Self {
        Self {
            f: DynamicScriptFunction::from(|_, _| ScriptValue::Unit),
            observer_on: None,
        }
    }
}

pub(super) struct VrmScriptsPlugin;

impl Plugin for VrmScriptsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VrmFindByNamePlugin,
            VrmSpawnPlugin,
            VrmRenderLayersPlugin,
            VrmMoveToPlugin,
            VrmSetStatePlugin,
            VrmRescalePlugin,
            VrmObserveOnPlugin,
            VrmOnLoadPlugin,
            VrmPointerPlugin,
            VrmOnTimerPlugin,
            VrmOnRangeTimerPlugin,
            // VrmOnCommandPlugin,
            VrmOnStateEnterPlugin,
            VrmSpeakPlugin,
            VrmObserverPlugin,
        ))
        .insert_non_send_resource(RunOnMainThread);
    }
}

#[derive(Reflect, Clone, Debug)]
pub(crate) struct VrmInstance {
    pub entity: Entity,
    pub observe_on: Option<VrmState>,
}

impl VrmInstance {
    pub fn new(vrm: Entity) -> Self {
        Self {
            entity: vrm,
            observe_on: None,
        }
    }

    pub fn create_handler(&self, f: DynamicScriptFunction) -> VrmHandlerBase {
        VrmHandlerBase {
            f,
            observer_on: self.observe_on.clone(),
        }
    }
}

#[macro_export]
macro_rules! vrm_handler {
    ($handler: ident) => {
        #[derive(Deref, Clone, Reflect, Default)]
        pub(crate) struct $handler(pub $crate::vrm::VrmHandlerBase);

        impl From<$crate::vrm::VrmHandlerBase> for $handler {
            fn from(base: $crate::vrm::VrmHandlerBase) -> Self {
                Self(base)
            }
        }
    };
}

// fn register_vrm_handler<H>(
//     ctx: FunctionCallContext,
//     me: ScriptVal<VrmInstance>,
//     f: DynamicScriptFunction,
// ) -> ScriptValueResult<VrmInstance>
// where
//     H: From<VrmHandlerBase> + Reflect + Default + Clone + 'static,
// {
//     let vrm_entity = me.0.entity;
//     let handler = me.create_handler(f);
//     ctx.world()?.with_global_access(move |world| {
//         if let Ok(mut handlers) = world
//             .query::<&mut VrmHandlers<H>>()
//             .get_mut(world, vrm_entity)
//         {
//             handlers.push(H::from(handler));
//         }
//     })?;
//     Ok(ScriptVal::new(me.clone()))
// }
