#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.9, 1.0);
    //return vec4<f32>(t);
    //return vec4<f32>(input.uv, 0.0, 1.0);
    //return vec4<f32>(input.world_normal + 0.0, 0.0);
    //return vec4<f32>(input.uv.xxx, 1.0);
}
