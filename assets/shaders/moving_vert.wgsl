#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

let TAU = 6.283185307179586;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var position = vertex.position;

    var out: VertexOutput;
    out.uv = vertex.uv;
    //out.uv.y += globals.time * 0.1;

    // local_to_world
    let world_position = mesh.model * vec4<f32>(position, 1.0);
    out.world_position = world_position;
    // world_to_clip
    let clip = view.view_proj * world_position;
    out.clip_position = clip;

    out.world_normal = vertex.normal;

    return out;
}


