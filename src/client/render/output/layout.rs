use nalgebra::Vector2;

use crate::client::render::util::Texture;

use super::{
    util::{ResizableTexture, Storage},
    WindowView, CHUNK_WIDTH,
};

pub struct Layout {
    render_bind_layout: wgpu::BindGroupLayout,
    render_pipeline_layout: wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    pub view: Storage,
    pub chunks: ResizableTexture,
}

pub const LABEL: &str = file!();

impl Layout {
    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        input: &Texture,
    ) -> Self {
        let view = Storage::init_with(device, "view", bytemuck::bytes_of(&WindowView::default()));

        let texture_desc = wgpu::TextureDescriptor {
            label: Some("chunks"),
            size: wgpu::Extent3d {
                width: CHUNK_WIDTH,
                height: CHUNK_WIDTH,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[wgpu::TextureFormat::R32Uint],
        };
        let view_desc = wgpu::TextureViewDescriptor {
            label: Some("chunk view"),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        };
        let chunks = ResizableTexture::new(device, texture_desc, view_desc);

        let render_bind_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    view.bind_group_layout_entry(
                        0,
                        true,
                        wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ),
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Uint,
                            view_dimension: wgpu::TextureViewDimension::D2Array,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some(LABEL),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(LABEL),
                bind_group_layouts: &[&render_bind_layout],
                push_constant_ranges: &[],
            });

        Self {
            view,
            chunks,
            render_bind_layout,
            render_pipeline_layout,
            format: config.format,
        }
    }

    pub fn bind_group(&self, device: &wgpu::Device, input: &Texture) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.render_bind_layout,
            entries: &[
                self.view.bind_group_entry(0),
                self.chunks.view_entry(1),
                input.view_bind_group_entry(2),
                input.sampler_bind_group_entry(3),
            ],
            label: Some(LABEL),
        })
    }

    pub fn pipeline(
        &self,
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(LABEL),
            layout: Some(&self.render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: true,
            },
            multiview: None,
            cache: None,
        })
    }
}
