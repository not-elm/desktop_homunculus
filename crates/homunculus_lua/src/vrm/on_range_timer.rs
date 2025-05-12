use crate::util::with_static_guard;
use crate::vrm::{VrmHandlerBase, VrmInstance};
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy_mod_scripting::core::bindings::{DynamicScriptFunction, FunctionCallContext};
use bevy_mod_scripting::lua::bindings::script_value::LUA_CALLER_CONTEXT;
use bevy_mod_scripting::script_bindings;
use homunculus_core::prelude::{OutputLog, VrmState};
use std::time::Duration;

#[derive(Reflect, Default, Component, Deref, DerefMut)]
#[reflect(Component)]
pub(crate) struct OnRangeTimerHandlers(pub Vec<RangeTimerHandler>);

#[derive(Reflect, Clone, Default)]
pub(crate) struct RangeTimerHandler {
    pub timer: Timer,
    pub min: Duration,
    pub max: Duration,
    pub base: VrmHandlerBase,
}

pub(super) struct VrmOnRangeTimerPlugin;

impl Plugin for VrmOnRangeTimerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RangeTimerHandler>()
            .register_type::<OnRangeTimerHandlers>()
            .add_systems(Update, (reset_timers, tick_timers).chain());

        register_vrm_on_range_timer_function(app.world_mut());
    }
}

#[script_bindings(name = "vrm_on_range_timer_function")]
#[allow(unused)]
impl VrmInstance {
    fn on_range_timer(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        min: ScriptVal<Duration>,
        max: ScriptVal<Duration>,
        mode: ScriptVal<TimerMode>,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<VrmInstance> {
        let vrm_entity = me.0.entity;
        let h = me.create_handler(f);
        let min = min.into_inner();
        let max = max.into_inner();
        ctx.world()?.with_global_access(move |world| {
            let Ok(mut handlers) = world
                .query::<&mut OnRangeTimerHandlers>()
                .get_mut(world, vrm_entity)
            else {
                return;
            };
            handlers.push(RangeTimerHandler {
                timer: Timer::new(rand::random_range(min..=max), mode.into_inner()),
                min,
                max,
                base: h,
            });
        })?;
        Ok(ScriptVal::new(me.clone()))
    }
}

fn tick_timers(world: &mut World) {
    let delta = world.resource::<Time>().delta();
    let mut callable_handlers = Vec::new();
    for (mut handlers, state) in world
        .query::<(&mut OnRangeTimerHandlers, &VrmState)>()
        .iter_mut(world)
    {
        for handler in handlers.iter_mut() {
            if !handler.base.target_state(state) {
                continue;
            }
            if handler.timer.tick(delta).just_finished() {
                handler
                    .timer
                    .set_duration(rand::random_range(handler.min..=handler.max));
                handler.timer.reset();
                callable_handlers.push(handler.base.f.clone());
            }
        }
    }
    with_static_guard(world, move || {
        for f in callable_handlers {
            f.call(vec![], LUA_CALLER_CONTEXT)
                .output_log_if_error("OnRangeTimer")
        }
    });
}

fn reset_timers(mut vrms: Query<&mut OnRangeTimerHandlers, Changed<VrmState>>) {
    for mut handlers in vrms.iter_mut() {
        for handler in handlers.iter_mut() {
            handler.timer.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vrm::VrmHandlerBase;
    use crate::vrm::on_range_timer::{
        OnRangeTimerHandlers, RangeTimerHandler, VrmOnRangeTimerPlugin,
    };
    use bevy::app::App;
    use bevy::prelude::*;
    use bevy_test_helper::error::TestResult;
    use homunculus_core::prelude::VrmState;
    use std::time::Duration;

    #[test]
    fn test_reset_timer() -> TestResult {
        let mut app = App::new();
        app.add_plugins((VrmOnRangeTimerPlugin, MinimalPlugins));

        app.world_mut().spawn((
            OnRangeTimerHandlers(vec![RangeTimerHandler {
                timer: Timer::from_seconds(3., TimerMode::Repeating),
                base: VrmHandlerBase::default(),
                ..default()
            }]),
            VrmState::default(),
        ));
        app.world_mut()
            .query::<&mut OnRangeTimerHandlers>()
            .single_mut(app.world_mut())?[0]
            .timer
            .tick(Duration::from_millis(1000));
        app.update();

        app.world_mut()
            .query::<&mut VrmState>()
            .single_mut(app.world_mut())?
            .0 = "test_state".to_string();
        app.update();

        let elapsed = app
            .world_mut()
            .query::<&mut OnRangeTimerHandlers>()
            .single(app.world_mut())?[0]
            .timer
            .elapsed();
        assert_eq!(elapsed, app.world().resource::<Time>().delta());
        Ok(())
    }
}
