// view uniform
#import bevy_pbr::mesh_view_bindings
// mesh uniform
#import bevy_pbr::mesh_bindings
//#import bevy_pbr::mesh_functions
//#import bevy_pbr::mesh_view_types
//#import bevy_pbr::mesh_types

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>,
//     @location(2) uv: vec2<f32>,
//     @location(3) tangent: vec4<f32>,
//     @location(4) color: vec4<f32>,
//     @location(5) joint_indices: vec4<u32>,
//     @location(6) joint_weights: vec4<f32>,
// };

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = vec4<f32>(vertex.position, 1.0);// idk
    var uv_scale: f32 = 1.0;
    var uv_offset: f32 = 0.0;//-0.0; //(globals.time % 5.0) * 0.05;
    out.uv = (vertex.uv + uv_offset) * uv_scale;

    // ~/prog/extern/bevy/crates/bevy_pbr/src/render/mesh_functions.wgsl
    // local_to_world
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    // world_to_clip
    let clip = view.view_proj * world_position;
    out.clip_position = clip;

    // mesh_normal_local_to_world
    // world normals is the weird one,
    // where the normal colors point in the same direction,
    // regardless of the object's orientation
    out.world_normal = normalize(
        mat3x3<f32>(
            mesh.inverse_transpose_model[0].xyz,
            mesh.inverse_transpose_model[1].xyz,
            mesh.inverse_transpose_model[2].xyz
        ) * vertex.normal
    );

    // this is what you usually want
    out.world_normal = vertex.normal;

    return out;
}
