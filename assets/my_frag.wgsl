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

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    //return vec4<f32>(input.world_normal + 0.0, 0.0);
    //return vec4<f32>(input.uv, 0.0, 1.0);
    //return vec4<f32>(input.uv.xxx, 1.0);

    // blend between two colors base on the X uv coordinate
    let color_a = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let color_b = vec4<f32>(0.0, 0.0, 1.0, 1.0);
    let color_start = 0.0;
    let color_end = 0.5;

    var t = inverse_lerp(color_start, color_end, input.uv.x);
    //t = clamp(t, 0.0, 1.0);
    //t = fract(t);
    //return vec4<f32>(t);

    let outColor = mix(color_a, color_b, t); // lerp
    return outColor;
    //return vec4<f32>(t);
}
