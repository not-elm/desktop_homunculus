use crate::mascot::Mascot;
use bevy::app::{App, Update};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub struct MascotRenderLayersPlugin;

impl Plugin for MascotRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_children_layers,).chain());
    }
}

fn update_children_layers(
    mut commands: Commands,
    mascots: Query<
        (&RenderLayers, &Children),
        (With<Mascot>, Or<(Changed<RenderLayers>, Added<Children>)>),
    >,
    meshes: Query<(Option<&RenderLayers>, Option<&Children>), Without<Mascot>>,
) {
    for (layers, children) in mascots.iter() {
        for child in children.iter() {
            replace_children_layers(child, layers.clone(), &mut commands, &meshes);
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
                replace_children_layers(child, render_layers.clone(), commands, meshes);
            }
        }
        if layers.is_some() {
            commands.entity(root_entity).insert(render_layers);
        }
    }
}
