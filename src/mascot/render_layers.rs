use crate::mascot::Mascot;
use crate::system_param::coordinate::Coordinate;
use bevy::app::{App, PreUpdate};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::debug;
use bevy::prelude::{Changed, Commands, Entity, ParallelCommands, Plugin, Query, Transform, With, Without};
use bevy::render::view::RenderLayers;

pub struct MascotRenderLayersPlugin;

impl Plugin for MascotRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, change_render_layers);
    }
}


fn change_render_layers(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &Name, &Transform, &RenderLayers, Option<&Children>), (Changed<Transform>, With<Mascot>)>,
    coordinate: Coordinate,
    meshes: Query<(Option<&RenderLayers>, Option<&Children>), Without<Mascot>>,
) {
    mascots.par_iter().for_each(|(entity, name, tf, layers, children)| {
        let Some((new_pos, new_layers)) = coordinate.new_render_layers_if_overall_monitor(
            layers,
            tf.translation,
        ) else {
            return;
        };
        let mut new_tf = *tf;
        new_tf.translation = new_pos;
        debug!("{name:?}'s render layer changed to {new_layers:?}");
        par_commands.command_scope(|mut commands| {
            if let Some(children) = children {
                for child in children.iter() {
                    update_layers(*child, new_layers.clone(), &mut commands, &meshes);
                }
            }
            commands.entity(entity).insert((
                new_layers.clone(),
                new_tf,
            ));
        });
    });
}

fn update_layers(
    root_entity: Entity,
    render_layers: RenderLayers,
    commands: &mut Commands,
    meshes: &Query<(Option<&RenderLayers>, Option<&Children>), Without<Mascot>>,
) {
    if let Ok((layers, children)) = meshes.get(root_entity) {
        if let Some(children) = children {
            for child in children.iter() {
                update_layers(*child, render_layers.clone(), commands, meshes);
            }
        }
        if layers.is_some() {
            commands.entity(root_entity).insert(render_layers);
        }
    }
}