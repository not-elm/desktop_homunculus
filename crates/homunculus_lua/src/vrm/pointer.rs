mod handler;

use crate::util::WorldEvents;
use crate::vrm::pointer::handler::VrmPointerHandlerPlugin;
use crate::vrm::{VrmHandlerBase, VrmHandlers, VrmInstance};
use crate::{ScriptVal, ScriptValueResult};
use bevy::prelude::*;
use bevy::render::camera::NormalizedRenderTarget;
use bevy_mod_scripting::core::bindings::{
    DynamicScriptFunction, FunctionCallContext, IntoScript, WorldGuard,
};
use bevy_mod_scripting::core::event::{IntoCallbackLabel, ScriptCallbackEvent};
use bevy_mod_scripting::core::handler::event_handler;
use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::script_bindings;
use bevy_vrm1::prelude::ParentSearcher;
use bevy_vrm1::vrm::{Initialized, Vrm};
use homunculus_core::prelude::{AppWindows, GlobalViewport, HomunculusSystemSet};
use std::fmt::Debug;
use std::ops::Deref;

pub(super) struct VrmPointerPlugin;

impl Plugin for VrmPointerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPointerPressed>()
            .add_event::<OnPointerMoved>()
            .add_event::<OnPointerReleased>()
            .add_event::<OnPointerDragStart>()
            .add_event::<OnPointerDrag>()
            .add_event::<OnPointerDragEnd>()
            .register_type::<OnPointerPressedHandler>()
            .register_type::<OnPointerMovedHandler>()
            .register_type::<OnPointerReleasedHandler>()
            .register_type::<OnPointerDragStartHandler>()
            .register_type::<OnPointerDragHandler>()
            .register_type::<OnPointerDragEndHandler>()
            .register_type::<VrmHandlers<OnPointerPressedHandler>>()
            .register_type::<VrmHandlers<OnPointerMovedHandler>>()
            .register_type::<VrmHandlers<OnPointerReleasedHandler>>()
            .register_type::<VrmHandlers<OnPointerDragStartHandler>>()
            .register_type::<VrmHandlers<OnPointerDragHandler>>()
            .register_type::<VrmHandlers<OnPointerDragEndHandler>>()
            .register_type::<PointerParams>()
            .add_plugins((VrmPointerHandlerPlugin,))
            .add_systems(Update, setup_observers)
            .add_systems(
                Update,
                (
                    fire_pointer_callbacks::<OnPointerPressed>.run_if(on_event::<OnPointerPressed>),
                    fire_pointer_callbacks::<OnPointerMoved>.run_if(on_event::<OnPointerMoved>),
                    fire_pointer_callbacks::<OnPointerReleased>
                        .run_if(on_event::<OnPointerReleased>),
                    fire_pointer_callbacks::<OnPointerDragStart>
                        .run_if(on_event::<OnPointerDragStart>),
                    fire_pointer_callbacks::<OnPointerDrag>.run_if(on_event::<OnPointerDrag>),
                    fire_pointer_callbacks::<OnPointerDragEnd>.run_if(on_event::<OnPointerDragEnd>),
                )
                    .before(HomunculusSystemSet::ScriptEventHandle),
            )
            .add_systems(
                Update,
                (
                    event_handler::<OnPointerPressed, LuaScriptingPlugin>,
                    event_handler::<OnPointerMoved, LuaScriptingPlugin>,
                    event_handler::<OnPointerReleased, LuaScriptingPlugin>,
                    event_handler::<OnPointerDragStart, LuaScriptingPlugin>,
                    event_handler::<OnPointerDrag, LuaScriptingPlugin>,
                    event_handler::<OnPointerDragEnd, LuaScriptingPlugin>,
                )
                    .in_set(HomunculusSystemSet::ScriptEventHandle),
            );

        register_functions(app.world_mut());
    }
}

#[derive(Reflect, Debug, Clone)]
pub struct PointerParams {
    pub target: Entity,
    pub vrm: String,
    pub viewport_pos: Vec2,
    pub screen_pos: Vec2,
    pub delta: Vec2,
}

impl Default for PointerParams {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            vrm: String::new(),
            viewport_pos: Vec2::ZERO,
            screen_pos: Vec2::ZERO,
            delta: Vec2::ZERO,
        }
    }
}

macro_rules! pointer_script_event {
    ($event: ident, $callback: ident) => {
        paste::paste! {
            #[derive(Reflect, Deref, Clone, Default)]
            pub(crate) struct [<$event Handler>](VrmHandlerBase);

            #[derive(Event, Reflect, Clone, Deref, Default)]
            pub(crate) struct $event(PointerParams);
            impl From<PointerParams> for $event {
                fn from(params: PointerParams) -> Self {
                    Self(params)
                }
            }
            impl bevy_mod_scripting::core::event::IntoCallbackLabel for $event {
                fn into_callback_label() -> bevy_mod_scripting::core::event::CallbackLabel {
                    stringify!($callback).into()
                }
            }
        }
    };
    ($([$event: ident, $callback: ident]),+ $(,)?) => {
        $(pointer_script_event!($event, $callback);)*

        paste::paste! {
            #[script_bindings]
            #[allow(unused)]
            impl VrmInstance {
                $(
                fn $callback(ctx: FunctionCallContext, me: ScriptVal<VrmInstance>, f: DynamicScriptFunction) -> ScriptValueResult<VrmInstance>{
                    let vrm_entity = me.0.entity;
                    let observer_on = me.0.observe_on.clone();
                    ctx.world()?.with_global_access(|world|{
                        if let Ok(mut handlers) = world.query::<&mut VrmHandlers<[<$event Handler>]>>().get_mut(world, vrm_entity){
                            handlers.push([<$event Handler>](VrmHandlerBase{
                                f,
                                observer_on,
                            }));
                        }
                    })?;
                    Ok(ScriptVal::new(me.clone()))
                })*
            }
        }
    };
}

pointer_script_event!(
    [OnPointerPressed, on_pointer_pressed],
    [OnPointerMoved, on_pointer_moved],
    [OnPointerReleased, on_pointer_released],
    [OnPointerDragStart, on_drag_start],
    [OnPointerDrag, on_drag],
    [OnPointerDragEnd, on_drag_end],
);

trait PointerDelta {
    fn delta(&self) -> Vec2 {
        Vec2::ZERO
    }
}
impl PointerDelta for Pressed {}
impl PointerDelta for Released {}
impl PointerDelta for Move {
    fn delta(&self) -> Vec2 {
        self.delta
    }
}
impl PointerDelta for DragStart {}
impl PointerDelta for Drag {
    fn delta(&self) -> Vec2 {
        self.delta
    }
}

impl PointerDelta for DragEnd {}

fn setup_observers(mut commands: Commands, vrms: Query<Entity, (Added<Initialized>, With<Vrm>)>) {
    for vrm in vrms.iter() {
        commands
            .entity(vrm)
            .observe(apply_write_pointer_event::<Pressed, OnPointerPressed>)
            .observe(apply_write_pointer_event::<Move, OnPointerMoved>)
            .observe(apply_write_pointer_event::<Released, OnPointerReleased>)
            .observe(apply_write_pointer_event::<DragStart, OnPointerDragStart>)
            .observe(apply_write_pointer_event::<Drag, OnPointerDrag>)
            .observe(apply_write_pointer_event::<DragEnd, OnPointerDragEnd>);
    }
}

fn apply_write_pointer_event<P, E>(
    trigger: Trigger<Pointer<P>>,
    mut ew: EventWriter<E>,
    parent_searcher: ParentSearcher,
    windows: AppWindows,
    vrms: Query<&Name>,
) where
    P: Debug + Clone + Reflect + PointerDelta,
    E: From<PointerParams> + Event,
{
    let Some(vrm_entity) = parent_searcher.find_vrm(trigger.target) else {
        return;
    };
    let Ok(vrm_name) = vrms.get(vrm_entity) else {
        return;
    };
    let Some(global_screen_pos) = global_cursor_pos(&trigger, &windows) else {
        return;
    };
    ew.write(E::from(PointerParams {
        target: vrm_entity,
        vrm: vrm_name.to_string(),
        viewport_pos: trigger.pointer_location.position,
        screen_pos: global_screen_pos.0,
        delta: trigger.delta(),
    }));
}

fn global_cursor_pos<E: Debug + Clone + Reflect>(
    trigger: &Trigger<Pointer<E>>,
    windows: &AppWindows,
) -> Option<GlobalViewport> {
    let NormalizedRenderTarget::Window(window_ref) = trigger.pointer_location.target else {
        return None;
    };
    windows.to_global_viewport(window_ref.entity(), trigger.pointer_location.position)
}

fn fire_pointer_callbacks<E>(world: &mut World)
where
    E: Event + IntoCallbackLabel + Deref<Target = PointerParams> + Clone,
{
    let events = world.read_all_events::<E>();
    let callbacks = WorldGuard::with_static_guard(world, |guard| {
        events.iter().flat_map(move |e| {
            let params = ScriptVal::new(e.deref().clone())
                .into_script(guard.clone())
                .ok()?;
            Some(ScriptCallbackEvent::new_for_all(
                E::into_callback_label(),
                vec![params],
            ))
        })
    });
    world.send_event_batch(callbacks);
}
