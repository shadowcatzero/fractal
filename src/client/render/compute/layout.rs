use wgpu::{PipelineCompilationOptions, ShaderStages};

use crate::client::render::util::ArrayBuffer;

use super::{
    util::{Storage, Texture},
    ComputeView,
};

pub struct Layout {
    bind_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    pub output: Texture,
    pub view: Storage,
    pub work: ArrayBuffer<u32>,
}

impl Layout {
    pub fn init(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, len: usize) -> Self {
        let view = Storage::init_with(device, "view", ComputeView::default().bytes());
        let work = ArrayBuffer::init_with(
            device,
            "test",
            wgpu::BufferUsages::STORAGE,
            &work_vec(config.width, config.height, len),
        );

        let desc = wgpu::TextureDescriptor {
            label: Some("compute output"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let output = Texture::init(
            device,
            desc,
            wgpu::TextureViewDescriptor::default(),
            wgpu::SamplerDescriptor::default(),
        );

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                view.bind_group_layout_entry(0, true, wgpu::ShaderStages::COMPUTE),
                work.bind_group_layout_entry(
                    1,
                    wgpu::BufferBindingType::Storage { read_only: false },
                    wgpu::ShaderStages::COMPUTE,
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: output.format(),
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            label: Some("compute"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Tile Pipeline Layout"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        Self {
            view,
            output,
            bind_layout,
            pipeline_layout,
            work,
        }
    }

    pub fn bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_layout,
            entries: &[
                self.view.bind_group_entry(0),
                self.work.bind_group_entry(1),
                self.output.view_bind_group_entry(2),
            ],
            label: Some("voxel render"),
        })
    }

    pub fn pipeline(
        &self,
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::ComputePipeline {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Voxel Pipeline"),
            layout: Some(&self.pipeline_layout),
            entry_point: Some("main"),
            module: shader,
            cache: None,
            compilation_options: PipelineCompilationOptions::default(),
        })
    }
}

pub fn work_size(width: u32, height: u32, len: usize) -> usize {
    let varwidth = (2 + len) * 2;
    (width * height) as usize * (varwidth * 2 + 1)
}

pub fn work_vec(width: u32, height: u32, len: usize) -> Vec<u32> {
    vec![0u32; work_size(width, height, len)]
}
