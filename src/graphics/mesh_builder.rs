use wgpu::util::DeviceExt;

type Polygon = [Vertex; 3];

#[rustfmt::skip]
const TRIANGLE: Polygon = [
        Vertex{ position: [-0.75, -0.75], color: [1.0, 0.0, 0.0 ] },
        Vertex{ position: [0.75, -0.75], color: [0.0, 1.0, 0.0 ] },
        Vertex{ position: [0.0, 0.75], color: [0.0, 0.0, 1.0 ] },
    ];

#[rustfmt::skip]
const QUAD: [Vertex; 6] = [
        Vertex{ position: [-0.5,  0.5], color: [1.0, 0.0, 0.0 ] },
        Vertex{ position: [ 0.5,  0.5], color: [0.0, 1.0, 0.0 ] },
        Vertex{ position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0 ] },

        Vertex{ position: [-0.5,  0.5], color: [0.0, 1.0, 0.0 ] },
        Vertex{ position: [ 0.5, -0.5], color: [0.0, 1.0, 0.0 ] },
        Vertex{ position: [-0.5, -0.5], color: [0.0, 0.0, 1.0 ] },
    ];

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

pub struct QuadMesh {
    mesh: [Vertex; 6],
    buf: wgpu::Buffer,
}

impl QuadMesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let mesh = QUAD;
        let buf = make_quad_buffer(device, &mesh);

        Self { mesh, buf }
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buf.slice(..)
    }
}

pub struct TriangleMesh {
    mesh: Polygon,
    buf: wgpu::Buffer,
    is_inverted: bool,
}

impl TriangleMesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let mesh = TRIANGLE;
        let buf = make_triangle_buffer(device, &mesh);

        Self {
            mesh,
            buf,
            is_inverted: false,
        }
    }

    pub fn is_inverted(&self) -> bool {
        self.is_inverted
    }

    pub fn invert(&mut self, device: &wgpu::Device) {
        self.mesh = invert_triangle(self.mesh);
        self.buf = make_triangle_buffer(device, &self.mesh);
        self.is_inverted = !self.is_inverted
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buf.slice(..)
    }
}

fn make_quad_buffer(device: &wgpu::Device, mesh: &[Vertex; 6]) -> wgpu::Buffer {
    let buf_disc = wgpu::util::BufferInitDescriptor {
        label: Some("quad buffer"),
        contents: &bytemuck::cast_slice(mesh),
        usage: wgpu::BufferUsages::VERTEX,
    };

    device.create_buffer_init(&buf_disc)
}

fn make_triangle_buffer(device: &wgpu::Device, mesh: &Polygon) -> wgpu::Buffer {
    let buf_disc = wgpu::util::BufferInitDescriptor {
        label: Some("triangle buffer"),
        contents: &bytemuck::cast_slice(mesh),
        usage: wgpu::BufferUsages::VERTEX,
    };

    device.create_buffer_init(&buf_disc)
}

fn invert_triangle(mut mesh: Polygon) -> Polygon {
    for vertex in mesh.iter_mut() {
        // Assuming position is [x, y, z]
        vertex.position[1] = -vertex.position[1]; // Flip Y coordinate
    }

    mesh.swap(0, 2);

    mesh
}
