use std::mem;

use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

use crate::game::{self, TileMap};

type Polygon = [Vertex; 3];

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector2<f32> = cgmath::Vector2::new(
    (NUM_INSTANCES_PER_ROW as f32 - 1.0) * 0.5,
    (NUM_INSTANCES_PER_ROW as f32 - 1.0) * 0.5,
);

#[rustfmt::skip]
const TRIANGLE: Polygon = [
        Vertex{ position: [-0.75, -0.75], color: [1.0, 0.0, 0.0 ], tex_coord: [0.0, 1.0]},
        Vertex{ position: [0.75, -0.75],  color: [0.0, 1.0, 0.0 ], tex_coord: [0.0, 1.0] },
        Vertex{ position: [0.0, 0.75],    color: [0.0, 0.0, 1.0 ], tex_coord: [0.0, 1.0] },
    ];

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
pub struct TileInstance {
    position: [f32; 2],
    texture_index: u32,
}

impl TileInstance {
    pub fn from_tile_map(tile_map: &TileMap) -> Vec<TileInstance> {
        tile_map
            .iter()
            .map(|tile| TileInstance {
                position: [tile.position.0 as f32, tile.position.1 as f32],
                texture_index: tile.ty as u32,
            })
            .collect()
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TileInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    shader_location: 4,
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

pub struct GridUniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl GridUniformBuffer {
    pub fn from(tile_map: &TileMap, device: &wgpu::Device) -> Self {
        let dims = tile_map.dimensions();
        let uniform: [f32; 2] = [dims.0 as f32, dims.1 as f32];
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

        Self { bind_group, bind_group_layout }
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
    mesh: [Vertex; 4],
    pub buf: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub instance_buf: wgpu::Buffer,
}

impl QuadMesh {
    pub fn new(device: &wgpu::Device, instances: &Vec<TileInstance>) -> Self {
        let mesh = QUAD;
        let (buf, index) = make_quad_buffers(device, &mesh);
        let instance_buf = make_instance_buffer(device, instances);

        Self {
            mesh,
            buf,
            index,
            instance_buf,
        }
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

fn make_instance_buffer(device: &wgpu::Device, instances: &Vec<TileInstance>) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("instance buffer"),
        contents: bytemuck::cast_slice(&instances),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    })
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
