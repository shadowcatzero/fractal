use std::marker::PhantomData;

use wgpu::util::DeviceExt;

pub struct Uniform<T> {
    buffer: wgpu::Buffer,
    ty: PhantomData<T>,
}

impl<T: Default + bytemuck::Pod> Uniform<T> {
    pub fn init(device: &wgpu::Device, name: &str) -> Self {
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(name.to_owned() + " Uniform Buf")),
                contents: bytemuck::cast_slice(&[T::default()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            ty: PhantomData,
        }
    }
}

impl<T: bytemuck::Pod> Uniform<T> {
    pub fn init_with(device: &wgpu::Device, name: &str, data: &[T]) -> Self {
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(name.to_owned() + " Uniform Buf")),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            ty: PhantomData,
        }
    }
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        data: T,
    ) {
        let slice = &[data];
        let mut view = belt.write_buffer(
            encoder,
            &self.buffer,
            0,
            unsafe {
                std::num::NonZeroU64::new_unchecked((slice.len() * std::mem::size_of::<T>()) as u64)
            },
            device,
        );
        view.copy_from_slice(bytemuck::cast_slice(slice));
    }
}

impl<T> Uniform<T> {
    pub fn bind_group_layout_entry(&self, binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX
                | wgpu::ShaderStages::FRAGMENT
                | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
    pub fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.buffer.as_entire_binding(),
        }
    }
}
