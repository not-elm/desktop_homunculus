use crate::mascot::Mascot;
use crate::system_param::cameras::Cameras;
use crate::system_param::mesh_aabb::MascotAabb;
use bevy::app::{App, PreUpdate, Update};
use bevy::core::Name;
use bevy::hierarchy::Children;
use bevy::log::debug;
use bevy::math::Vec2;
use bevy::prelude::{Added, Changed, Commands, Entity, Or, ParallelCommands, Plugin, Query, Transform, Vec3Swizzles, With, Without};
use bevy::render::view::RenderLayers;

pub struct MascotRenderLayersPlugin;

impl Plugin for MascotRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, change_render_layers)
            .add_systems(Update, update_children_layers);
    }
}

fn change_render_layers(
    par_commands: ParallelCommands,
    mascots: Query<(Entity, &Name, &Transform, &RenderLayers), (Changed<Transform>, With<Mascot>)>,
    cameras: Cameras,
    mascot_aabb: MascotAabb,
) {
    mascots.par_iter().for_each(|(entity, name, tf, current_layers)| {
        let Some((_, current_gtf, _)) = cameras.find_camera_from_layers(current_layers) else {
            return;
        };
        let (min, max) = mascot_aabb.calculate(entity);
        let Some((min_camera, min_gtf, min_layers)) = cameras.find_camera_from_world_pos(min) else {
            return;
        };
        // let Some((_, max_gtf, max_layers)) = cameras.find_camera_from_world_pos(max) else {
        //     return;
        // };
        if current_layers == min_layers {
            return;
        }
        let mut new_tf = *tf;
        new_tf.translation = min_camera.viewport_to_world_2d(min_gtf, Vec2::ZERO).unwrap().extend(0.);
        // new_tf.translation += (min_gtf.translation().xy() - current_gtf.translation().xy())
        //     .normalize()
        //     .extend(0.);
        debug!("{name:?}'s render layer changed to {min_layers:?}");
        par_commands.command_scope(|mut commands| {
            // commands.entity(entity).insert((
            //     min_layers.clone(),
            //     new_tf,
            // ));
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