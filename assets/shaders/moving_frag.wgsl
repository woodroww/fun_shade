#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;
@group(1) @binding(3)
var pattern_texture: texture_2d<f32>;
@group(1) @binding(4)
var rock_texture: texture_2d<f32>;

fn get_wave(coord: vec4<f32>) -> vec4<f32> {
    let TAU = 6.283185307179586;
    var wave = cos((coord - globals.time * 0.1) * TAU * 5.0) * 0.5 + 0.5;
    wave *= 1.0 - coord;
    return wave;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    //return vec4<f32>(0.0, 0.0, 0.9, 1.0);

    //return textureSample(color_texture, color_sampler, input.uv);

    // repeate texture in world space projected top down, good for terrain
    var topDownProjection = input.world_position.xz;
    //return vec4<f32>(topDownProjection, 0.0, 1.0);
    //return textureSample(color_texture, color_sampler, topDownProjection);
    var moss =  textureSample(color_texture, color_sampler, topDownProjection);
    var rock = textureSample(rock_texture, color_sampler, topDownProjection);
    var pattern = textureSample(pattern_texture, color_sampler, input.uv).xxxx;

    //var finalColor = mix(vec4<f32>(1.0, 0.0, 0.0, 1.0), moss, pattern);
    var finalColor = mix(rock, moss, pattern);

    //return pattern;
    return finalColor;
    //return get_wave(pattern);

    // world position colors
    //return vec4<f32>(input.world_position.xyz, 1.0);
}
