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
pub(crate) struct OnTimerHandlers(pub Vec<TimerHandler>);

#[derive(Reflect, Clone)]
pub(crate) struct TimerHandler {
    pub timer: Timer,
    pub base: VrmHandlerBase,
}

pub(super) struct VrmOnTimerPlugin;

impl Plugin for VrmOnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TimerHandler>()
            .register_type::<OnTimerHandlers>()
            .add_systems(Update, (tick_timers, reset_timers));

        register_timer_mode_functions(app.world_mut());
        register_vrm_on_timer_function(app.world_mut());
    }
}

#[script_bindings(name = "timer_mode_functions", remote)]
#[allow(unused)]
impl TimerMode {
    fn repeating() -> ScriptVal<TimerMode> {
        ScriptVal::new(TimerMode::Repeating)
    }

    fn once() -> ScriptVal<TimerMode> {
        ScriptVal::new(TimerMode::Once)
    }
}

#[script_bindings(name = "vrm_on_timer_function")]
#[allow(unused)]
impl VrmInstance {
    fn on_timer(
        ctx: FunctionCallContext,
        me: ScriptVal<VrmInstance>,
        duration: ScriptVal<Duration>,
        mode: ScriptVal<TimerMode>,
        f: DynamicScriptFunction,
    ) -> ScriptValueResult<VrmInstance> {
        let vrm_entity = me.0.entity;
        let h = me.create_handler(f);
        ctx.world()?.with_global_access(move |world| {
            let Ok(mut handlers) = world
                .query::<&mut OnTimerHandlers>()
                .get_mut(world, vrm_entity)
            else {
                return;
            };
            handlers.push(TimerHandler {
                timer: Timer::new(duration.into_inner(), mode.into_inner()),
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
        .query::<(&mut OnTimerHandlers, &VrmState)>()
        .iter_mut(world)
    {
        for handler in handlers.iter_mut() {
            if !handler.base.target_state(state) {
                continue;
            }
            let timer = handler.timer.tick(delta);
            if timer.just_finished() {
                callable_handlers.push(handler.base.f.clone());
            }
        }
    }
    with_static_guard(world, move || {
        for f in callable_handlers {
            f.call(vec![], LUA_CALLER_CONTEXT)
                .output_log_if_error("OnTimer")
        }
    });
}

fn reset_timers(mut vrms: Query<&mut OnTimerHandlers, Changed<VrmState>>) {
    for mut handlers in vrms.iter_mut() {
        for handler in handlers.iter_mut() {
            handler.timer.reset();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vrm::VrmHandlerBase;
    use crate::vrm::on_timer::{OnTimerHandlers, TimerHandler, VrmOnTimerPlugin};
    use bevy::app::App;
    use bevy::prelude::*;
    use bevy_test_helper::error::TestResult;
    use homunculus_core::prelude::VrmState;
    use std::time::Duration;

    #[test]
    fn test_reset_timer() -> TestResult {
        let mut app = App::new();
        app.add_plugins(VrmOnTimerPlugin);

        app.world_mut().spawn((
            OnTimerHandlers(vec![TimerHandler {
                timer: Timer::from_seconds(3., TimerMode::Repeating),
                base: VrmHandlerBase::default(),
            }]),
            VrmState::default(),
        ));
        app.world_mut()
            .query::<&mut OnTimerHandlers>()
            .single_mut(app.world_mut())?[0]
            .timer
            .tick(Duration::from_millis(100));
        app.update();

        app.world_mut()
            .query::<&mut VrmState>()
            .single_mut(app.world_mut())?
            .0 = "test_state".to_string();
        app.update();

        let elapsed = app
            .world_mut()
            .query::<&mut OnTimerHandlers>()
            .single(app.world_mut())?[0]
            .timer
            .elapsed_secs();
        assert_eq!(elapsed, 0.0);
        Ok(())
    }
}
