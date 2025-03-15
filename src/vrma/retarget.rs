mod expressions;
mod bone;

use crate::vrma::retarget::bone::VrmaRetargetingBonePlugin;
use crate::vrma::retarget::expressions::VrmaRetargetExpressionsPlugin;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Changed, Component, Entity, EventWriter, IntoSystemConfigs, Query, SystemSet, Transform, With};
use bevy::window::RequestRedraw;


#[derive(SystemSet, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct RetargetBindingSystemSet;

#[derive(Component)]
pub struct CurrentRetargeting;

pub struct VrmaRetargetPlugin;

impl Plugin for VrmaRetargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                VrmaRetargetingBonePlugin,
                VrmaRetargetExpressionsPlugin,
            ))
            .add_systems(Update, request_redraw.run_if(playing_animation));
    }
}

fn playing_animation(
    changed_bones: Query<Entity, (Changed<Transform>, With<CurrentRetargeting>)>,
) -> bool {
    !changed_bones.is_empty()
}

fn request_redraw(
    mut request: EventWriter<RequestRedraw>,
) {
    request.send(RequestRedraw);
}