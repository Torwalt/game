use super::assets::LoadedImage;

struct Sprite {
    pub bind_group: wgpu::BindGroup,
}

impl Sprite {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, image: LoadedImage) -> Self {
        let texture_size = wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1,
        };

        let texture_discriptor = wgpu::TextureDescriptor {
            label: Some(&image.file_name),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        };

        let texture = device.create_texture(&texture_discriptor);
    }
}
