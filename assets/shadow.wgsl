#import bevy_pbr::{
    pbr_types,
    pbr_bindings,
    mesh_view_bindings::view,
    mesh_view_types,
    lighting,
    lighting::{LAYER_BASE, LAYER_CLEARCOAT},
    transmission,
    clustered_forward as clustering,
    shadows,
    ambient,
    irradiance_volume,
    mesh_types::{MESH_FLAGS_SHADOW_RECEIVER_BIT, MESH_FLAGS_TRANSMITTED_SHADOW_RECEIVER_BIT},
    forward_io::VertexOutput,
}
#import bevy_pbr::pbr_fragment::pbr_input_from_vertex_output
#import bevy_pbr::pbr_functions::apply_pbr_lighting
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material

@fragment
fn fragment(
    vertex: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    let in = pbr_input_from_standard_material(vertex, is_front);
    let view_z = dot(vec4<f32>(
            view.view_from_world[0].z,
            view.view_from_world[1].z,
            view.view_from_world[2].z,
            view.view_from_world[3].z
        ), in.world_position);
    let shadow = apply_pbr_lighting(in);
    if(0.99 < shadow.r && 0.99 < shadow.g && 0.99 < shadow.b) {
        discard;
    }
    return vec4(vec3(0.), 0.4);
}