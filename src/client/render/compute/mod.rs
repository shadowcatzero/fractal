use std::ops::{Deref, DerefMut};

mod data;
mod layout;

use super::*;
pub use data::*;
use layout::*;

pub struct ComputePipeline {
    layout: Layout,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    old_view: ComputeView,
    old_len: usize,
}

const FIXED_SHADER: &str = include_str!("fixed.wgsl");
const SHADER: &str = include_str!("shader.wgsl");

impl ComputePipeline {
    pub fn init(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, len: usize) -> Self {
        let layout = Layout::init(device, config, len);
        Self {
            pipeline: layout.pipeline(device, &Self::shader(device, len)),
            bind_group: layout.bind_group(device),
            layout,
            old_view: ComputeView::default(),
            old_len: len,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        camera: &Camera,
        size: &Vector2<u32>,
        len: usize,
    ) {
        let mut view = ComputeView::from_camera_size(camera, size, false, len);
        if view != self.old_view {
            for (i, b) in 1u32.to_le_bytes().iter().enumerate() {
                view.bytes[i] = *b;
            }
        }
        if len != self.old_len {
            println!("len: {}", len);
            self.old_len = len;
            self.pipeline = self.pipeline(device, &Self::shader(device, len));
            self.work.set(work_vec(size.x, size.y, len));
        }
        let updated = self.work.update(device, encoder, belt)
            | self.view.update(device, encoder, belt, view.bytes());
        if updated {
            self.bind_group = self.layout.bind_group(device);
        }
        self.old_view = view;
    }

    pub fn run(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.dispatch_workgroups(240, 135, 1);
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        size: Vector2<u32>,
        len: usize,
    ) {
        self.work.set(work_vec(size.x, size.y, len));
        self.old_len = len;
        self.output.resize(
            device,
            wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
        );
        self.bind_group = self.layout.bind_group(device);
    }

    pub fn shader(device: &wgpu::Device, len: usize) -> wgpu::ShaderModule {
        let string = FIXED_SHADER.to_string() + &SHADER.replace("REPLACE_LEN", &format!("{}", len));
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute"),
            source: wgpu::ShaderSource::Wgsl(string.into()),
        })
    }
}

impl Deref for ComputePipeline {
    type Target = Layout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

impl DerefMut for ComputePipeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout
    }
}
