mod compute;
mod output;
mod util;
mod view;

use std::sync::Arc;

use compute::ComputePipeline;
use nalgebra::Vector2;
use output::RenderPipeline;
use util::GPUTimer;
use view::ChunkView;
use winit::{dpi::PhysicalSize, window::Window};

use super::camera::Camera;

const CHUNK_POW: u32 = 7;
const CHUNK_WIDTH: u32 = 2u32.pow(CHUNK_POW);

pub struct Renderer<'a> {
    size: Vector2<u32>,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    encoder: wgpu::CommandEncoder,
    config: wgpu::SurfaceConfiguration,
    staging_belt: wgpu::util::StagingBelt,
    timer: GPUTimer,
    chunk_view: ChunkView,
    len: usize,

    compute_pipeline: ComputePipeline,
    render_pipeline: RenderPipeline,
}

impl Renderer<'_> {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window)
            .expect("Could not create window surface!");

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Could not get adapter!");

        let buf_size = (10f32.powi(9) * 1.5) as u32;
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::PUSH_CONSTANTS
                    | wgpu::Features::TIMESTAMP_QUERY
                    | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS
                    | wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES,
                required_limits: wgpu::Limits {
                    max_storage_buffer_binding_size: buf_size,
                    max_buffer_size: buf_size as u64,
                    max_push_constant_size: 4,
                    ..Default::default()
                },
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))
        .expect("Could not get device!");

        let info = adapter.get_info();
        println!("Adapter: {}", info.name);
        println!("Backend: {:?}", info.backend);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let timer = GPUTimer::new(&device, queue.get_timestamp_period(), 1);

        let len = 2;

        let compute_pipeline = ComputePipeline::init(&device, &config, len);
        let render_pipeline = RenderPipeline::init(&device, &config, &compute_pipeline.output);

        Self {
            render_pipeline,
            compute_pipeline,
            size: Vector2::new(size.width, size.height),
            staging_belt,
            surface,
            encoder: Self::create_encoder(&device),
            timer,
            device,
            config,
            queue,
            chunk_view: ChunkView::new(),
            len,
        }
    }

    pub fn render(&mut self, camera: &Camera) {

        self.len = (camera.zoom.level() as f32 / 15.0 + 2.0).round() as usize;
        println!("{}", self.len);
        // let new = (camera.zoom.level() as f32 / 15.0 + 2.0).round() as usize;
        // println!("{}", new);

        self.compute_pipeline.update(
            &self.device,
            &mut self.encoder,
            &mut self.staging_belt,
            camera,
            &self.size,
            self.len,
        );
        self.chunk_view.update(camera, &self.size);
        self.render_pipeline.update(
            &self.device,
            &mut self.encoder,
            &mut self.staging_belt,
            &self.chunk_view.render,
        );

        let mut encoder = std::mem::replace(&mut self.encoder, Self::create_encoder(&self.device));
        let output = self.surface.get_current_texture().unwrap();

        self.timer.start(&mut encoder, 0);
        self.compute_pipeline.run(&mut encoder);
        self.timer.stop(&mut encoder, 0);
        self.timer.resolve(&mut encoder);

        self.render_pipeline.draw(&mut encoder, &output);

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();

        self.timer.finish(&self.device);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = Vector2::new(size.width, size.height);
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.compute_pipeline.resize(&self.device, &mut self.encoder, &mut self.staging_belt, self.size, self.len);
        self.render_pipeline.resize(&self.device, self.size, &self.compute_pipeline.output);
    }

    fn create_encoder(device: &wgpu::Device) -> wgpu::CommandEncoder {
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        })
    }
}
