use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

#[rustfmt::skip]
const QUAD: [Vertex; 4] = [
    // tex_coords map texture corners to corner of quad. Because the quad is essentially a canvas,
    // we can use the corner verteces as texture corners. However, we need to make sure to use the
    // correct coordinate system.
    // Top Left.
    Vertex{ position: [-0.5,  0.5], color: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] },
    // Bottom Left.
    Vertex{ position: [-0.5, -0.5], color: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
    // Bottom Right.
    Vertex{ position: [ 0.5, -0.5], color: [0.0, 0.0, 1.0], tex_coord: [1.0, 1.0] },
    // Top Right.
    Vertex{ position: [ 0.5,  0.5], color: [0.0, 1.0, 0.0], tex_coord: [1.0, 0.0] },
];

pub const QUAD_INDEX: [u32; 6] = [0, 1, 2, 3, 2, 0];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct Instance {
    pub position: [f32; 2],
    pub texture_index: u32,
    pub z_order: f32,
    pub entity_type: u32,
    pub animation_frame: u32,
    pub frame_pos_offset: [f32; 2],
}

impl Instance {
    const ATTRS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
            // position
            3 => Float32x2,
            // texture_index
            4 => Uint32,
            // z_order
            5 => Float32,
            // entity_type
            6 => Uint32,
            // animation_frame
            7 => Uint32,
            // frame_pos_offset
            8 => Float32x2
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem::size_of;
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRS,
        }
    }
}

pub struct GridUniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl GridUniformBuffer {
    pub fn from(width: f32, height: f32, device: &wgpu::Device) -> Self {
        let uniform: [f32; 2] = [width, height];
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("grid_bind_group_layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("grid_bind_group"),
        });

        Self {
            bind_group,
            bind_group_layout,
        }
    }
}

pub struct Camera {
    view: cgmath::Matrix4<f32>,
    orthographic: cgmath::Matrix4<f32>,
    world_width: f32,
}

impl Camera {
    #[rustfmt::skip]
    pub fn new(width: f32, height: f32, world_width: f32) -> Self {
        println!("width: {} height: {}", width, height);

        let view = cgmath::Matrix4::identity();
        let orthographic = Camera::create_ortho(width, height, world_width);

        Self { orthographic, view, world_width }
    }

    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.orthographic = Camera::create_ortho(width as f32, height as f32, self.world_width);
    }

    fn create_ortho(width: f32, height: f32, world_width: f32) -> cgmath::Matrix4<f32> {
        let aspect = width / height;
        let world_height = world_width / aspect;

        cgmath::ortho(
            -world_width / 2.0,
            world_width / 2.0,
            -world_height / 2.0,
            world_height / 2.0,
            -1.0,
            1.0,
        )
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        self.orthographic * self.view
    }
}

pub struct CameraBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl CameraBuffer {
    pub fn new(camera: &Camera, device: &wgpu::Device) -> Self {
        let view_proj = camera.build_view_projection_matrix();
        let uniform: [[f32; 4]; 4] = view_proj.into();
        println!("camera uniform: {:?}", uniform);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            bind_group,
            bind_group_layout,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
    tex_coord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3, 2 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

pub struct QuadMesh {
    pub buf: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub instance_buf: wgpu::Buffer,
}

impl QuadMesh {
    pub fn new(device: &wgpu::Device, instances: &Vec<Instance>) -> Self {
        let mesh = QUAD;
        let (buf, index) = make_quad_buffers(device, &mesh);
        let instance_buf = make_instance_buffer(device, instances);

        Self {
            buf,
            index,
            instance_buf,
        }
    }
}

fn make_quad_buffers(device: &wgpu::Device, mesh: &[Vertex; 4]) -> (wgpu::Buffer, wgpu::Buffer) {
    let buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("quad buffer"),
        contents: &bytemuck::cast_slice(mesh),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("index buffer"),
        contents: &bytemuck::cast_slice(&QUAD_INDEX),
        usage: wgpu::BufferUsages::INDEX,
    });

    (buf, index)
}

fn make_instance_buffer(device: &wgpu::Device, instances: &Vec<Instance>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("instance buffer"),
        contents: bytemuck::cast_slice(&instances),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
}
