use crate::mascot::action::MascotActionExt;
use crate::mascot::MascotEntity;
use bevy::prelude::*;
use bevy_flurx::action::wait;
use bevy_flurx::prelude::Omit;
use bevy_vrm1::vrma::animation::AnimationPlayerEntityTo;

pub struct WaitAnimationPlugin;

impl WaitAnimationPlugin {
    pub const ID: &'static str = "wait_animation";
}

impl Plugin for WaitAnimationPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_mascot_action(Self::ID, |mascot, _: ()| {
            wait::until(all_animation_finished).with(mascot).omit()
        });
    }
}

fn all_animation_finished(
    In(mascot): In<MascotEntity>,
    children: Query<&Children>,
    vrma: Query<&AnimationPlayerEntityTo>,
    player: Query<&AnimationPlayer>,
) -> bool {
    children.get(mascot.0).is_ok_and(|children| {
        children
            .iter()
            .filter_map(|c| vrma.get(c).ok())
            .filter_map(|p| player.get(p.0).ok())
            .all(|p| p.all_finished())
    })
}
