struct InstanceInput {
    @location(3) instance_position: vec2<f32>, // Matches pipeline layout
}

struct Vertex {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) texCoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texCoord: vec2<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var myTexture: texture_2d<f32>;
@group(0) @binding(1) var mySampler: sampler;
@group(1) @binding(0) var<uniform> camera: CameraUniform;

@vertex
fn vs_main(vertex: Vertex, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    out.texCoord = vertex.texCoord;

    let translation = vec4(instance.instance_position, 1.0, 1.0);
    let world_pos = vec4(vertex.position, 0.0, 1.0) + translation;
    // let world_pos = vec4(vertex.position, 0.0, 1.0);
    // out.clip_position = camera.view_proj * world_pos; // Apply camera matrix
    out.clip_position = world_pos; // Apply camera matrix

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(myTexture, mySampler, in.texCoord);
}
