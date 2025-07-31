use crate::util::with_static_guard;
use crate::vrm::{VrmHandlerBase, VrmHandlers, VrmInstance};
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, FunctionCallContext};
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::{OutputLog, VrmState};

pub(super) struct VrmOnStateEnterPlugin;

impl Plugin for VrmOnStateEnterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, call_handlers.run_if(any_changed_mascot_states));
        register_vrm_on_state_enter_function(app.world_mut());
    }
}

#[derive(Clone, Reflect, Default)]
pub(crate) struct OnStateEnterHandler {
    pub base: VrmHandlerBase,
    pub state: VrmState,
}

impl OnStateEnterHandler {
    pub fn target_state(&self, current: &VrmState) -> bool {
        self.base.target_state(current) && self.state == *current
    }
}

#[script_bindings(name = "vrm_on_state_enter_function")]
#[allow(unused)]
impl VrmInstance {
    fn on_state_enter(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        state: String,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<VrmInstance> {
        let vrm_entity = me.0.entity;
        let handler = me.create_handler(f);
        ctx.world()?.with_global_access(move |world| {
            if let Ok(mut handlers) = world
                .query::<&mut VrmHandlers<OnStateEnterHandler>>()
                .get_mut(world, vrm_entity)
            {
                handlers.push(OnStateEnterHandler {
                    base: handler,
                    state: VrmState(state),
                });
            }
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}

fn any_changed_mascot_states(
    vrms: Query<&VrmHandlers<OnStateEnterHandler>, Changed<VrmState>>,
) -> bool {
    if vrms.is_empty() {
        return false;
    }
    vrms.iter().any(|handlers| !handlers.0.is_empty())
}

fn call_handlers(world: &mut World) {
    let handlers = collect_on_state_enter_handlers(world);
    with_static_guard(world, || {
        for f in handlers {
            f.call(vec![], LUA_CALLER_CONTEXT)
                .output_log_if_error("on_state_enter");
        }
    });
}

fn collect_on_state_enter_handlers(world: &mut World) -> Vec<DynamicScriptFunction> {
    world
        .query_filtered::<(&VrmState, &VrmHandlers<OnStateEnterHandler>), Changed<VrmState>>()
        .iter(world)
        .flat_map(|(state, handlers)| {
            handlers.iter().flat_map(move |h: &OnStateEnterHandler| {
                h.target_state(state).then_some(h.base.f.clone())
            })
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::vrm::VrmHandlers;
    use crate::vrm::on_state_enter::{
        OnStateEnterHandler, VrmOnStateEnterPlugin, collect_on_state_enter_handlers,
    };
    use bevy::prelude::*;
    use bevy_test_helper::error::TestResult;
    use homunculus_core::prelude::VrmState;

    #[test]
    fn test_collect_on_state_enter_handlers() -> TestResult {
        let mut app = App::new();
        app.add_plugins(VrmOnStateEnterPlugin);

        let vrm = spawn_vrm(&mut app, "state1");
        app.world_mut()
            .query::<&mut VrmState>()
            .get_mut(app.world_mut(), vrm)?
            .0 = "state1".to_string();

        let handlers = collect_on_state_enter_handlers(app.world_mut());
        assert_eq!(handlers.len(), 1);

        Ok(())
    }

    #[test]
    fn test_not_target() -> TestResult {
        let mut app = App::new();
        app.add_plugins(VrmOnStateEnterPlugin);

        let vrm = spawn_vrm(&mut app, "unknown");
        app.world_mut()
            .query::<&mut VrmState>()
            .get_mut(app.world_mut(), vrm)?
            .0 = "state1".to_string();

        let handlers = collect_on_state_enter_handlers(app.world_mut());
        assert_eq!(handlers.len(), 0);

        Ok(())
    }

    fn spawn_vrm(app: &mut App, state: &str) -> Entity {
        app.world_mut()
            .spawn((
                VrmState::default(),
                VrmHandlers(vec![OnStateEnterHandler {
                    state: VrmState(state.to_string()),
                    ..default()
                }]),
            ))
            .id()
    }
}
