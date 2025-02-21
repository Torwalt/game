struct InstanceInput {
    @location(3) position: vec2<f32>,
    @location(4) texture_index: u32,
    @location(5) z_order: f32,
    @location(6) entity_type: u32,
    @location(7) animation_frame: u32,
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
    @location(2) entity_type: u32,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct GridUniform {
    @location(0) dimensions: vec2<f32>
}

@group(0) @binding(0) var floor_texture: texture_2d<f32>;
@group(0) @binding(1) var floor_texture_sampler: sampler;

@group(1) @binding(0) var wall_texture: texture_2d<f32>;
@group(1) @binding(1) var wall_texture_sampler: sampler;

@group(2) @binding(0) var<uniform> camera: CameraUniform;
@group(3) @binding(0) var<uniform> grid: GridUniform;

@group(4) @binding(0) var monster_texture: texture_2d<f32>;
@group(4) @binding(1) var monster_texture_sampler: sampler;

@vertex
fn vs_main(vertex: Vertex, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    out.texCoord = vertex.texCoord;
    out.texture_index = instance.texture_index;
    out.entity_type = instance.entity_type;

    let grid_center = grid.dimensions / 2.0;
    let centered_instance_pos = instance.position - grid_center;

    // Create world space position
    let world_pos = vec4(
        vertex.position + centered_instance_pos,
        0,
        1.0
    );

    // Apply camera transformation
    out.clip_position = camera.view_proj * world_pos;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.entity_type == 0u) {
        if (in.texture_index == 0u) {
            return textureSample(floor_texture, floor_texture_sampler, in.texCoord);
        }

        return textureSample(wall_texture, wall_texture_sampler, in.texCoord);
    }

    if (in.entity_type == 1u) {
        return textureSample(monster_texture, monster_texture_sampler, in.texCoord);
    }

    return vec4<f32>(1.0, 0.0, 1.0, 1.0); // Bright pink for debugging
}

