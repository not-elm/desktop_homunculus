use crate::ScriptVal;
use bevy::animation::RepeatAnimation;
use bevy::prelude::*;
use bevy_mod_scripting::script_bindings;

pub(super) struct RepeatPlugin;

impl Plugin for RepeatPlugin {
    fn build(&self, app: &mut App) {
        register_repeat_animation_functions(app.world_mut());
    }
}

#[script_bindings(name = "repeat_animation_functions", remote)]
#[allow(unused)]
impl RepeatAnimation {
    fn count(count: u32) -> ScriptVal<RepeatAnimation> {
        ScriptVal::new(RepeatAnimation::Count(count))
    }

    fn forever() -> ScriptVal<RepeatAnimation> {
        ScriptVal::new(RepeatAnimation::Forever)
    }

    fn never() -> ScriptVal<RepeatAnimation> {
        ScriptVal::new(RepeatAnimation::Never)
    }
}
