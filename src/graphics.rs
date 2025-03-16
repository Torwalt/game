use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use pollster::FutureExt;
use wgpu::{Adapter, Device, PresentMode, Queue, Surface, SurfaceCapabilities};
use winit::dpi::{self, PhysicalSize};
use winit::window::Window;

use crate::game::{self, GameState, TileMap};

use self::sprites::Sprite;

pub mod assets;
pub mod mesh_builder;
pub mod sprites;

pub struct State {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    clear_color: wgpu::Color,

    render_pipeline: wgpu::RenderPipeline,
    quad_mesh: mesh_builder::QuadMesh,
    floor_tile: Sprite,
    wall_tile: Sprite,
    monster: Sprite,
    instances: Vec<mesh_builder::Instance>,
    camera_buffer: mesh_builder::CameraBuffer,
    camera: mesh_builder::Camera,
    grid_uniform_buffer: mesh_builder::GridUniformBuffer,
}

impl State {
    pub fn new(window: Window, assets_path: PathBuf) -> Self {
        let window_arc = Arc::new(window);
        let size = window_arc.inner_size();
        let instance = Self::create_gpu_instance();
        let surface = instance
            .create_surface(window_arc.clone())
            .context("when creating instance")
            .unwrap();
        let adapter = Self::create_adapter(instance, &surface);

        println!("Using Adapter: {:?}", adapter.get_info().name);
        println!("Device Type: {:?}", adapter.get_info().device_type);

        let (device, queue) = Self::create_device(&adapter);
        let surface_caps = surface.get_capabilities(&adapter);
        let config = Self::create_surface_config(size, surface_caps);
        surface.configure(&device, &config);
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let loaded_floor_tile =
            assets::LoadedImage::from_path(&assets_path, "sprites/test5.png").unwrap();
        let floor_tile = sprites::Sprite::new(&device, &queue, loaded_floor_tile);

        let loaded_wall_tile =
            assets::LoadedImage::from_path(&assets_path, "sprites/test4.png").unwrap();
        let wall_tile = sprites::Sprite::new(&device, &queue, loaded_wall_tile);

        let loaded_monster =
            assets::LoadedImage::from_path(&assets_path, "sprites/goblin-spritemap.png").unwrap();
        let monster = sprites::Sprite::new(&device, &queue, loaded_monster);

        let camera = mesh_builder::Camera::new(size.width as f32, size.height as f32, 25.0);
        let camera_buffer = mesh_builder::CameraBuffer::new(&camera, &device);

        let dims = (20, 20);
        let grid_uniform_buffer =
            mesh_builder::GridUniformBuffer::from(dims.0 as f32, dims.1 as f32, &device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &floor_tile.bind_group_layout,
                    &wall_tile.bind_group_layout,
                    &camera_buffer.bind_group_layout,
                    &grid_uniform_buffer.bind_group_layout,
                    &monster.bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[mesh_builder::Vertex::desc(), mesh_builder::Instance::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let empty_instances: Vec<mesh_builder::Instance> = Vec::new();
        let quad_mesh = mesh_builder::QuadMesh::new(&device, &empty_instances);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window: window_arc,
            clear_color: wgpu::Color::BLACK,
            render_pipeline,
            quad_mesh,
            floor_tile,
            wall_tile,
            monster,
            instances: vec![],
            camera_buffer,
            camera,
            grid_uniform_buffer,
        }
    }

    fn create_surface_config(
        size: PhysicalSize<u32>,
        capabilities: SurfaceCapabilities,
    ) -> wgpu::SurfaceConfiguration {
        let surface_format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    fn create_device(adapter: &Adapter) -> (Device, Queue) {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    memory_hints: wgpu::MemoryHints::Performance,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits {
                        max_bind_groups: 5,
                        ..Default::default()
                    },
                    label: None,
                },
                None,
            )
            .block_on()
            .unwrap()
    }

    fn create_adapter(instance: wgpu::Instance, surface: &Surface) -> Adapter {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap()
    }

    fn create_gpu_instance() -> wgpu::Instance {
        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    pub fn resize(&mut self, new_size: dpi::PhysicalSize<u32>) {
        self.camera
            .update_aspect_ratio(new_size.width, new_size.height);
        self.camera_buffer = mesh_builder::CameraBuffer::new(&self.camera, &self.device);
    }

    pub fn update(&mut self, s: &GameState) -> Result<()> {
        let instances: Vec<mesh_builder::Instance> = s.instances();
        let quad_mesh = mesh_builder::QuadMesh::new(&self.device, &instances);
        self.quad_mesh = quad_mesh;
        self.instances = instances;

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.floor_tile.bind_group, &[]);
            render_pass.set_bind_group(1, &self.wall_tile.bind_group, &[]);
            render_pass.set_bind_group(2, &self.camera_buffer.bind_group, &[]);
            render_pass.set_bind_group(3, &self.grid_uniform_buffer.bind_group, &[]);
            render_pass.set_bind_group(4, &self.monster.bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.quad_mesh.buf.slice(..));
            render_pass.set_vertex_buffer(1, self.quad_mesh.instance_buf.slice(..));
            render_pass.set_index_buffer(self.quad_mesh.index.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(
                0..mesh_builder::QUAD_INDEX.len() as u32,
                0,
                0..self.instances.len() as u32,
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
