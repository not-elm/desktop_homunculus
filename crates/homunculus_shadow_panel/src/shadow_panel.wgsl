#import bevy_pbr::{
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
    mesh_view_bindings::{view, lights},
    shadows::fetch_directional_shadow,
    mesh_types::{MESH_FLAGS_SHADOW_RECEIVER_BIT, MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT},
    forward_io::VertexOutput,
    pbr_fragment::pbr_input_from_vertex_output,
}

@group(2) @binding(100) var<uniform> alpha_factor: f32;

@fragment
fn fragment(
    vertex: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let in = pbr_input_from_vertex_output(vertex, is_front, false);
    let view_z = dot(vec4<f32>(
        view.view_from_world[0].z,
        view.view_from_world[1].z,
        view.view_from_world[2].z,
        view.view_from_world[3].z
    ), in.world_position);
    var shadow = 0.0;
    let n_directional_lights = lights.n_directional_lights;
    for (var i: u32 = 0u; i < lights.n_directional_lights; i = i + 1u) {
        if((lights.directional_lights[i].flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) == 0u ){
            continue;
        }
        shadow += fetch_directional_shadow(i, in.world_position, in.world_normal, view_z);
    }
    return vec4(vec3(0.), (1. - shadow) * alpha_factor);
}
