use crate::vrm::VrmInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{
    DynamicScriptFunction, FunctionCallContext, ThreadWorldContainer, WorldAccessGuard,
    WorldContainer,
};
use bevy_mod_scripting::core::callback_labels;
use bevy_mod_scripting::core::event::ScriptCallbackEvent;
use bevy_mod_scripting::core::handler::event_handler;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::{Initialized, Vrm};
use core::convert::Into;
use homunculus_core::prelude::OutputLog;

pub(super) struct VrmOnLoadPlugin;

impl Plugin for VrmOnLoadPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnLoadHandler>().add_systems(
            Update,
            (
                fire_on_load_vrm,
                call_on_load_handlers.run_if(any_initialized_vrm),
                event_handler::<OnLoadVrm, LuaScriptingPlugin>,
            )
                .chain(),
        );

        register_vrm_on_load_function(app.world_mut());
    }
}

callback_labels!(
    OnLoadVrm => "on_load_vrm",
);

#[derive(Reflect, Deref, DerefMut, Component, Clone)]
#[reflect(Component)]
struct OnLoadHandler(DynamicScriptFunction);

#[script_bindings(name = "vrm_on_load_function")]
#[allow(unused)]
impl VrmInstance {
    fn on_load(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<VrmInstance> {
        let vrm_entity = me.0.entity;
        ctx.world()?.with_global_access(move |world| {
            world.commands().entity(vrm_entity).insert(OnLoadHandler(f));
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}

fn fire_on_load_vrm(
    mut ew: EventWriter<ScriptCallbackEvent>,
    vrms: Query<&Name, (With<Vrm>, Added<Initialized>)>,
) {
    ew.write_batch(vrms.iter().map(|vrm_name| {
        ScriptCallbackEvent::new_for_all(OnLoadVrm, vec![vrm_name.to_string().into()])
    }));
}

fn call_on_load_handlers(world: &mut World) {
    let vrms = world
        .query_filtered::<(&Name, &OnLoadHandler), Added<Initialized>>()
        .iter(world)
        .map(|(n, h)| (n.clone(), h.clone()))
        .collect::<Vec<_>>();
    WorldAccessGuard::with_static_guard(world, move |world| {
        ThreadWorldContainer.set_world(world).unwrap();
        for (vrm_name, handler) in vrms {
            handler
                .call(vec![vrm_name.to_string().into()], LUA_CALLER_CONTEXT)
                .output_log_if_error("OnLoad");
        }
    });
}

fn any_initialized_vrm(vrms: Query<&Vrm, Added<Initialized>>) -> bool {
    !vrms.is_empty()
}
