use crate::util::{json_to_script_value, with_static_guard, WorldEvents};
use crate::vrm::{VrmHandlerBase, VrmHandlers, VrmInstance};
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, FunctionCallContext, ScriptValue};
use bevy_mod_scripting::core::callback_labels;
use bevy_mod_scripting::core::event::ScriptCallbackEvent;
use bevy_mod_scripting::core::handler::event_handler;
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::{HomunculusSystemSet, OutputLog, VrmState};
use homunculus_http_server::prelude::HomunculusCommand;

pub(super) struct VrmOnCommandPlugin;

impl Plugin for VrmOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnCommandHandler>()
            .add_systems(
                Update,
                send_script_callback_event.before(HomunculusSystemSet::ScriptEventHandle),
            )
            .add_systems(
                Update,
                event_handler::<OnCommandLabel, LuaScriptingPlugin>
                    .in_set(HomunculusSystemSet::ScriptEventHandle),
            )
            .add_systems(Update, call_handlers.run_if(on_event::<HomunculusCommand>));
        register_vrm_on_command_function(app.world_mut());
    }
}

callback_labels!(
    OnCommandLabel => "on_command",
);

#[derive(Clone, Reflect, Default)]
pub(crate) struct OnCommandHandler {
    base: VrmHandlerBase,
    command: String,
}

#[script_bindings(name = "vrm_on_command_function")]
#[allow(unused)]
impl VrmInstance {
    fn on_command(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        command: String,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<VrmInstance> {
        let vrm_entity = me.0.entity;
        let handler = me.create_handler(f);
        ctx.world()?.with_global_access(move |world| {
            if let Ok(mut handlers) = world
                .query::<&mut VrmHandlers<OnCommandHandler>>()
                .get_mut(world, vrm_entity)
            {
                handlers.push(OnCommandHandler {
                    base: handler,
                    command,
                });
            }
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}

fn send_script_callback_event(
    mut er: EventReader<HomunculusCommand>,
    mut ew: EventWriter<ScriptCallbackEvent>,
) {
    ew.write_batch(er.read().map(|e| {
        let mut args = Vec::new();
        args.push(ScriptValue::from(e.command.clone()));
        args.extend(convert_args(e));
        ScriptCallbackEvent::new_for_all(OnCommandLabel, args)
    }));
}

fn call_handlers(world: &mut World) {
    let events = world.read_all_events::<HomunculusCommand>();
    let handlers = world
        .query::<(&VrmState, &VrmHandlers<OnCommandHandler>)>()
        .iter(world)
        .flat_map(|(s, h)| {
            h.iter()
                .flat_map(|h| h.base.target_state(s).then_some(h.clone()))
        })
        .collect::<Vec<_>>();
    with_static_guard(world, || {
        for event in events {
            for handler in handlers.iter().filter(|h| h.command == event.command) {
                handler
                    .base
                    .f
                    .call(convert_args(&event), LUA_CALLER_CONTEXT)
                    .output_log_if_error("on_command");
            }
        }
    });
}

fn convert_args(event: &HomunculusCommand) -> Vec<ScriptValue> {
    match event.args.clone() {
        Some(value) => {
            vec![json_to_script_value(value)]
        }
        None => Vec::new(),
    }
}
