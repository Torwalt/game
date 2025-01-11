@group(0) @binding(0) var our_sampler: sampler;
@group(0) @binding(1) var our_texture: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index) / 2);
    let y = f32(i32(in_vertex_index) % 2);
    out.clip_position = vec4(x * 2.0 - 1.0, y * -2.0 + 1.0, 0.0, 1.0);
    out.tex_coords = vec2(x, y);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(our_texture, our_sampler, in.tex_coords);
}
