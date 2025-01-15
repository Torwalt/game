use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    #[rustfmt::skip]
    let verts: [Vertex; 3] = [
        Vertex{ position: [-0.75, -0.75], color: [1.0, 0.0, 0.0 ] },
        Vertex{ position: [0.75, -0.75], color: [0.0, 1.0, 0.0 ] },
        Vertex{ position: [0.0, 0.75], color: [0.0, 0.0, 1.0 ] },
    ];

    let buf_disc = wgpu::util::BufferInitDescriptor {
        label: Some("triangle buffer"),
        contents: &bytemuck::cast_slice(&verts),
        usage: wgpu::BufferUsages::VERTEX,
    };

    device.create_buffer_init(&buf_disc)
}