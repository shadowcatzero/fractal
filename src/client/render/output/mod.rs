use std::ops::{Deref, DerefMut};
use wgpu::include_wgsl;

mod data;
mod layout;

use super::{util::Texture, *};
pub use data::*;
use layout::*;

pub struct RenderPipeline {
    layout: Layout,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    size: Vector2<u32>,
}

const SHADER: wgpu::ShaderModuleDescriptor<'_> = include_wgsl!("shader.wgsl");

impl RenderPipeline {
    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        input: &Texture,
    ) -> Self {
        let layout = Layout::init(device, config, input);
        let shader = device.create_shader_module(SHADER);
        Self {
            pipeline: layout.pipeline(device, &shader),
            bind_group: layout.bind_group(device, input),
            size: Vector2::zeros(),
            layout,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        view: &WindowView,
    ) {
        self.view
            .update(device, encoder, belt, bytemuck::bytes_of(view));
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, output: &wgpu::SurfaceTexture) {
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..4, 0..1);
    }

    pub fn resize(&mut self, device: &wgpu::Device, size: Vector2<u32>, input: &Texture) {
        self.bind_group = self.layout.bind_group(device, input);
    }
}

impl Deref for RenderPipeline {
    type Target = Layout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

impl DerefMut for RenderPipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout
    }
}
