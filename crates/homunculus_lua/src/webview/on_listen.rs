use crate::util::{WorldEvents, json_to_script_value, with_static_guard};
use crate::vrm::VrmHandlers;
use crate::webview::WebviewInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, FunctionCallContext};
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::script_bindings;
use bevy_webview_wry::prelude::IpcTriggerExt;
use homunculus_core::prelude::OutputLog;
use serde::{Deserialize, Serialize};

pub(super) struct WebviewOnListenPlugin;

impl Plugin for WebviewOnListenPlugin {
    fn build(&self, app: &mut App) {
        app.add_ipc_trigger::<ScriptWebviewMessage>("script-event")
            .add_systems(Update, on_listen.run_if(on_event::<ScriptWebviewMessage>));

        register_webview_on_listen(app.world_mut());
    }
}

#[derive(Serialize, Deserialize, Event, Clone, Debug)]
pub(crate) struct ScriptWebviewMessage {
    pub id: String,
    pub args: serde_json::Value,
}

pub struct WebviewScriptHandler {
    event_id: String,
    function: DynamicScriptFunction,
}

fn on_listen(world: &mut World) {
    let events = world.read_all_events::<ScriptWebviewMessage>();
    let handlers = world
        .query::<&VrmHandlers<WebviewScriptHandler>>()
        .iter(world)
        .flat_map(|handlers| handlers.0.iter())
        .flat_map(|handler| {
            let event = events.iter().find(|e| e.id == handler.event_id)?;
            Some((handler.function.clone(), event.args.clone()))
        })
        .collect::<Vec<_>>();
    with_static_guard(world, || {
        for (function, args) in handlers {
            function
                .call(vec![json_to_script_value(args)], LUA_CALLER_CONTEXT)
                .output_log_if_error("on_listen");
        }
    });
}

#[script_bindings(name = "webview_on_listen")]
#[allow(unused)]
impl WebviewInstance {
    fn on_listen(
        ctx: FunctionCallContext,
        me: ScriptVal<WebviewInstance>,
        event_id: String,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<WebviewInstance> {
        let entity = me.0.0;
        ctx.world()?.with_global_access(|world| {
            if let Ok(mut handlers) = world
                .query::<&mut VrmHandlers<WebviewScriptHandler>>()
                .get_mut(world, entity)
            {
                handlers.push(WebviewScriptHandler {
                    event_id,
                    function: f,
                });
            }
        })?;
        Ok(me)
    }
}
