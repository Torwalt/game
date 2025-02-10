struct InstanceInput {
    @location(3) instance_position: vec2<f32>,
    @location(4) texture_index: u32,
}

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) texCoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texCoord: vec2<f32>,
    @location(1) texture_index: u32,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var floor_texture: texture_2d<f32>;
@group(0) @binding(1) var floor_texture_sampler: sampler;

@group(1) @binding(0) var wall_texture: texture_2d<f32>;
@group(1) @binding(1) var wall_texture_sampler: sampler;

@vertex
fn vs_main(vertex: Vertex, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    out.texCoord = vertex.texCoord;
    out.texture_index = instance.texture_index;

    let scaled_position = vertex.position * 1;
    // let max_coord = 10.0; // Assuming this is the maximum coordinate in both x and y for now.
    // let translation_offset = max_coord / 2.0;
    let translation = vec4(instance.instance_position - vec2<f32>(4.5), 0.0, 1.0);
    let world_pos = vec4(scaled_position, 0.0, 1.0) + translation;

    // Convert to clip space
    /*
    let viewport_width = 20.0; // Example viewport width in world units
    let instances_per_row = 5.0; // How many instances you want in one row
    let scale_factor = viewport_width / (instances_per_row * max_coord);
    */
    let scaled = world_pos * vec4(0.4, 0.4, 0.0, 1.0);
    out.clip_position = scaled;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.texture_index == 0u) {
        return textureSample(floor_texture, floor_texture_sampler, in.texCoord);
    }

    return textureSample(wall_texture, wall_texture_sampler, in.texCoord);
}

