#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

// smooth step is similar to this but with a curve, this is a straight line
fn inverse_lerp(start: f32, end: f32, input: f32) -> f32 {
    return (input-start)/(end-start);
}

// gaurentees we go through an entire period of sin cos and such
// 2 * pi
let TAU = 6.283185307179586;

fn gradient(uv: vec2<f32>) -> vec4<f32> {
    // blend between two colors base on the X uv coordinate
    let color_a = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let color_b = vec4<f32>(0.0, 0.0, 1.0, 1.0);
    let color_start = 0.0;
    let color_end = 1.0;

    var t = inverse_lerp(color_start, color_end, uv.x);
    //t = clamp(t, 0.0, 1.0);
    let outColor = mix(color_a, color_b, t); // lerp

    return outColor;
}

fn cosine_stripes(uv: vec2<f32>) -> vec4<f32> {
    let stripe_count = 5.0;
    var t = cos(uv.x * TAU * stripe_count);
    return vec4<f32>(t);
}

fn both_ways_cosine(uv: vec2<f32>) -> vec4<f32> {
    let stripe_count = 5.0;
    var t = cos(uv.xy * TAU * stripe_count);
    return vec4<f32>(t, 0.0, 1.0);
}

fn diagonal(uv: vec2<f32>) -> vec4<f32> {
    // adding x and y to get diagonal
    let stripe_count = 5.0;
    let x_offset = uv.y;
    var t = cos((uv.x + x_offset) * TAU * stripe_count);
    return vec4<f32>(t);
}

fn cylinder_gradient(uv: vec2<f32>) -> vec4<f32> {
    // blend between two colors base on the X uv coordinate
    let color_a = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let color_b = vec4<f32>(0.0, 0.0, 1.0, 0.0);
    let color_start = 0.0;
    let color_end = 1.0;

    var t = inverse_lerp(color_start, color_end, uv.y);
    //t = clamp(t, 0.0, 1.0);
    let outColor = mix(color_a, color_b, t); // lerp

    return outColor;
}

fn moving_waves(input: VertexOutput) -> vec4<f32> {

    var xOffset = cos(input.uv.x * TAU * 8.0) * 0.02;
    var t = cos((input.uv.y + xOffset - globals.time * 0.1) * TAU * 5.0) * 0.5 + 0.5;
    t *= 1.0 - input.uv.y;
    return vec4<f32>(t);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return moving_waves(input) * cylinder_gradient(input.uv);
    //return cosine_stripes(input.uv);
    //return both_ways_cosine(input.uv);

    //var t = abs(fract(input.uv.x * 5.0) * 2.0 - 1.0);

    //var outColor = moving_waves(input);
    //return outColor;

    //return vec4<f32>(t);
    //return vec4<f32>(input.uv, 0.0, 1.0);
    //return vec4<f32>(input.world_normal + 0.0, 0.0);
    //return vec4<f32>(input.uv.xxx, 1.0);
}
