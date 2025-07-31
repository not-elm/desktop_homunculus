use bevy::prelude::*;
use bevy::render::view::{NoFrustumCulling, RenderLayers};
use bevy_vrm1::vrm::Vrm;
use bevy_vrm1::vrma::Vrma;

pub struct VrmRenderLayersPlugin;

impl Plugin for VrmRenderLayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_children_layers,).chain());
    }
}

fn update_children_layers(
    mut commands: Commands,
    mascots: Query<
        (&RenderLayers, &Children),
        (With<Vrm>, Or<(Changed<RenderLayers>, Added<Children>)>),
    >,
    meshes: Query<(Has<Mesh3d>, Option<&Children>), (Without<Vrm>, Without<Vrma>)>,
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
    meshes: &Query<(Has<Mesh3d>, Option<&Children>), (Without<Vrm>, Without<Vrma>)>,
) {
    if let Ok((has_mesh_3d, children)) = meshes.get(root_entity) {
        if let Some(children) = children {
            for child in children.iter() {
                replace_children_layers(child, render_layers.clone(), commands, meshes);
            }
        }
        if has_mesh_3d {
            commands
                .entity(root_entity)
                .insert((render_layers, NoFrustumCulling));
        }
    }
}
