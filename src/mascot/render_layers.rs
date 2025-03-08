use crate::mascot::Mascot;
use crate::system_param::cameras::Cameras;
use crate::system_param::mesh_aabb::MascotAabb;
use bevy::app::{App, PostUpdate, Update};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::debug;
use bevy::prelude::{apply_deferred, Added, Changed, Commands, Entity, IntoSystem, IntoSystemConfigs, Or, ParallelCommands, Plugin, Query, Transform, With, Without};
use bevy::render::view::RenderLayers;

pub struct MascotRenderLayersPlugin;

impl Plugin for MascotRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, (
                change_render_layers.pipe(apply_deferred),
                update_children_layers,
            ).chain());
    }
}

fn change_render_layers(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &Name, &RenderLayers), (Changed<Transform>, With<Mascot>)>,
    cameras: Cameras,
    mascot_aabb: MascotAabb,
) {
    mascots.par_iter().for_each(|(entity, name, current_layers)| {
        let (min, max) = mascot_aabb.calculate(entity);
        let Some((_, _, min_layer)) = cameras.find_camera_from_world_pos(min) else {
            return;
        };
        let Some((_, _, max_layer)) = cameras.find_camera_from_world_pos(max) else {
            return;
        };
        if current_layers == min_layer || (min_layer != max_layer) {
            return;
        }
        debug!("{name:?}'s render layer changed from {current_layers:?} to {min_layer:?}");
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert((
                min_layer.clone(),
            ));
        });
    });
}

fn update_children_layers(
    mut commands: Commands,
    mascots: Query<(&RenderLayers, &Children), (With<Mascot>, Or<(Changed<RenderLayers>, Added<Children>)>)>,
    meshes: Query<(Option<&RenderLayers>, Option<&Children>), Without<Mascot>>,
) {
    for (layers, children) in mascots.iter() {
        for child in children.iter() {
            replace_children_layers(*child, layers.clone(), &mut commands, &meshes);
        }
    }
}

fn replace_children_layers(
    root_entity: Entity,
    render_layers: RenderLayers,
    commands: &mut Commands,
    meshes: &Query<(Option<&RenderLayers>, Option<&Children>), Without<Mascot>>,
) {
    if let Ok((layers, children)) = meshes.get(root_entity) {
        if let Some(children) = children {
            for child in children.iter() {
                replace_children_layers(*child, render_layers.clone(), commands, meshes);
            }
        }
        if layers.is_some() {
            commands.entity(root_entity).insert(render_layers);
        }
    }
}