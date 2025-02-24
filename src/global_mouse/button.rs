use crate::global_mouse::GlobalMouseHandle;
use bevy::app::{App, PostUpdate, PreUpdate};
use bevy::input::ButtonInput;
use bevy::prelude::{Local, Plugin, Res, ResMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GlobalMouseButton {
    Left,
}

pub struct GlobalMouseButtonPlugin;

impl Plugin for GlobalMouseButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ButtonInput::<GlobalMouseButton>::default())
            .add_systems(PreUpdate, update_input)
            .add_systems(PostUpdate, clear_just_pressed);
    }
}

fn update_input(
    mut input: ResMut<ButtonInput<GlobalMouseButton>>,
    mut is_left_pressing: Local<bool>,
    global_mouse: Res<GlobalMouseHandle>,
) {
    if global_mouse.pressed_left() {
        input.press(GlobalMouseButton::Left);
        *is_left_pressing = true;
    } else if *is_left_pressing {
        input.release(GlobalMouseButton::Left);
        *is_left_pressing = false;
    }
}

fn clear_just_pressed(mut input: ResMut<ButtonInput<GlobalMouseButton>>) {
    input.clear_just_pressed(GlobalMouseButton::Left);
    input.clear_just_released(GlobalMouseButton::Left);
}