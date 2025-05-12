use crate::util::script_value_to_json;
use crate::webview::WebviewInstance;
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{FunctionCallContext, ScriptValue};
use bevy_mod_scripting::script_bindings;
use bevy_webview_wry::prelude::{EmitIpcEvent, EventPayload};

pub(super) struct WebviewSendPlugin;

impl Plugin for WebviewSendPlugin {
    fn build(&self, app: &mut App) {
        register_webview_send(app.world_mut());
    }
}

#[script_bindings(name = "webview_send")]
#[allow(unused)]
impl WebviewInstance {
    fn send(
        ctx: FunctionCallContext,
        me: ScriptVal<WebviewInstance>,
        event_id: String,
        args: ScriptValue,
    ) -> ScriptValueResult<WebviewInstance> {
        let entity = me.0.0;
        let args_json = script_value_to_json(&args);
        ctx.world()?.with_global_access(|world| {
            world.commands().entity(entity).trigger(EmitIpcEvent {
                id: event_id,
                payload: EventPayload::new(args_json),
            });
        })?;
        Ok(me)
    }
}
