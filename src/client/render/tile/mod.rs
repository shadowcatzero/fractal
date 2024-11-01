use std::ops::{Deref, DerefMut};

use wgpu::include_wgsl;

mod data;
mod layout;

use super::*;
pub use data::*;
use layout::*;

pub struct TilePipeline {
    layout: TileLayout,
    render_pipeline: wgpu::RenderPipeline,
    render_bind_group: wgpu::BindGroup,
}

const RENDER_SHADER: wgpu::ShaderModuleDescriptor<'_> = include_wgsl!("render.wgsl");

impl TilePipeline {
    pub fn init(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let layout = TileLayout::init(device, config);
        let render_shader = device.create_shader_module(RENDER_SHADER);
        Self {
            render_pipeline: layout.render_pipeline(device, render_shader),
            render_bind_group: layout.render_bind_group(device),
            layout,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        camera: &Camera,
        size: &Vector2<u32>,
    ) {
        self.view.update(device, encoder, belt, View::from_camera_size(camera, size).bytes());
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.render_bind_group, &[]);
        render_pass.draw(0..4, 0..1);
    }
}

impl Deref for TilePipeline {
    type Target = TileLayout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

impl DerefMut for TilePipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout
    }
}
