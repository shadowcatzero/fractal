use super::{util::Storage, View};

pub struct TileLayout {
    render_bind_layout: wgpu::BindGroupLayout,
    render_pipeline_layout: wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    pub view: Storage,
}

impl TileLayout {
    pub fn init(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let view = Storage::init_with(
            device,
            "view",
            View::default().bytes(),
        );

        let render_bind_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[view.bind_group_layout_entry(0, true)],
                label: Some("voxel render"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Tile Pipeline Layout"),
                bind_group_layouts: &[&render_bind_layout],
                push_constant_ranges: &[],
            });

        Self {
            view,
            render_bind_layout,
            render_pipeline_layout,
            format: config.format,
        }
    }

    pub fn render_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.render_bind_layout,
            entries: &[self.view.bind_group_entry(0)],
            label: Some("voxel render"),
        })
    }

    pub fn render_pipeline(
        &self,
        device: &wgpu::Device,
        shader: wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Voxel Pipeline"),
            layout: Some(&self.render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
