#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

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

    // local_to_world
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    // world_to_clip
    let clip = view.view_proj * world_position;
    out.clip_position = clip;

    out.world_normal = vertex.normal;

    return out;
}
