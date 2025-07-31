use crate::util::with_static_guard;
use crate::vrm::VrmInstance;
use crate::{ScriptResult, ScriptVal, ScriptValueResult};
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, FunctionCallContext, ScriptValue};
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::ChildSearcher;
use std::collections::VecDeque;
use std::time::Duration;

pub(super) struct FlowScriptPlugin;

impl Plugin for FlowScriptPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Flows>()
            .register_type::<Action>()
            .init_resource::<Flows>()
            .add_systems(Update, advance_action.run_if(any_flow));

        register_action_functions(app.world_mut());
        register_flow_functions(app.world_mut());
    }
}

#[derive(Reflect)]
pub struct Flow;

#[script_bindings(name = "flow_functions")]
#[allow(unused)]
impl Flow {
    fn will(ctx: FunctionCallContext, flow: ScriptVal<Action>) -> ScriptResult<()> {
        ctx.world()?.with_global_access(|world| {
            world.resource_mut::<Flows>().push(flow.into_inner());
        })?;
        Ok(())
    }
}

#[derive(Resource, Reflect, Deref, DerefMut, Default)]
pub struct Flows(pub Vec<Action>);

#[derive(Reflect, Component, Deref, DerefMut, Clone)]
pub struct Action(VecDeque<DynamicScriptFunction>);

#[derive(Reflect, Component, Deref, DerefMut, Clone)]
pub struct DelayTimer(Timer);

#[script_bindings(name = "action_functions")]
#[allow(unused)]
impl Action {
    fn wait_until(f: DynamicScriptFunction) -> ScriptVal<Action> {
        ScriptVal::new(Action(VecDeque::from([f])))
    }

    fn delay(ctx: FunctionCallContext, duration: ScriptVal<Duration>) -> ScriptValueResult<Action> {
        let duration = duration.into_inner();
        let timer_entity = ctx.world()?.with_global_access(|world| {
            world
                .commands()
                .spawn(DelayTimer(Timer::new(duration, TimerMode::Once)))
                .id()
        })?;

        Ok(Action::wait_until(DynamicScriptFunction::from(
            move |ctx: FunctionCallContext, _| {
                ctx.world()
                    .and_then(|guard| {
                        guard.with_global_access(|world| {
                            let delta = world.resource::<Time>().delta();
                            let Ok(mut timer) = world
                                .query::<&mut DelayTimer>()
                                .get_mut(world, timer_entity)
                            else {
                                return ScriptValue::Bool(true);
                            };
                            if timer.tick(delta).just_finished() {
                                world.commands().entity(timer_entity).despawn();
                                ScriptValue::Bool(true)
                            } else {
                                ScriptValue::Bool(false)
                            }
                        })
                    })
                    .unwrap_or(ScriptValue::Bool(true))
            },
        )))
    }

    fn once(f: DynamicScriptFunction) -> ScriptVal<Action> {
        Action::wait_until(DynamicScriptFunction::from(
            move |ctx: FunctionCallContext, _| {
                ctx.world()
                    .and_then(|_| {
                        f.call(vec![], LUA_CALLER_CONTEXT)?;
                        Ok(ScriptValue::Bool(true))
                    })
                    .unwrap_or(ScriptValue::Bool(true))
            },
        ))
    }

    fn wait_animation(vrm: ScriptVal<VrmInstance>) -> ScriptVal<Action> {
        let vrm_entity = vrm.0.entity;
        let f = DynamicScriptFunction::from(move |ctx: FunctionCallContext, _| {
            ctx.world()
                .and_then(|world| {
                    world.with_global_access(|world| {
                        world
                            .run_system_once_with(animation_finished, vrm_entity)
                            .unwrap_or_default()
                    })
                })
                .unwrap_or_default()
                .into()
        });
        Action::wait_until(f)
    }

    fn chain(me: ScriptVal<Action>, other: ScriptVal<Action>) -> ScriptVal<Action> {
        let mut me = me.into_inner();
        let other = other.into_inner();
        me.extend(other.0);
        ScriptVal::new(me)
    }
}

fn animation_finished(
    In(vrm): In<Entity>,
    players: Query<&AnimationPlayer>,
    childrens: Query<&Children>,
    searcher: ChildSearcher,
) -> bool {
    if let Some(expressions_root) = searcher.find_expressions_root(vrm) {
        if let Ok(children) = childrens.get(expressions_root) {
            for child in children.iter() {
                if let Ok(player) = players.get(child) {
                    if !player.all_finished() {
                        return false;
                    }
                }
            }
        }
    }
    let Some(root_bone) = searcher.find_root_bone(vrm) else {
        return true;
    };
    if let Ok(player) = players.get(root_bone) {
        player.all_finished()
    } else {
        true
    }
}

fn any_flow(flows: Res<Flows>) -> bool {
    !flows.0.is_empty()
}

fn advance_action(world: &mut World) {
    world.resource_scope(|world, mut flows: Mut<Flows>| {
        with_static_guard(world, || {
            flows.retain_mut(|flow| {
                let Some(action) = flow.0.pop_front() else {
                    return false;
                };
                let finished_action =
                    action
                        .call(vec![], LUA_CALLER_CONTEXT)
                        .is_ok_and(|r| match r {
                            ScriptValue::Bool(b) => b,
                            _ => true,
                        });
                if finished_action {
                    !flow.0.is_empty()
                } else {
                    flow.0.push_front(action);
                    true
                }
            });
        })
    });
}
