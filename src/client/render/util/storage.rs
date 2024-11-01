use std::marker::PhantomData;

use wgpu::util::DeviceExt;

pub struct Storage {
    buffer: wgpu::Buffer,
}

impl Storage {
    pub fn init<T: Default + bytemuck::Pod>(device: &wgpu::Device, name: &str) -> Self {
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(name.to_owned() + " Uniform Buf")),
                contents: bytemuck::cast_slice(&[T::default()]),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        }
    }
    pub fn init_with(device: &wgpu::Device, name: &str, data: &[u8]) -> Self {
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(name.to_owned() + " Uniform Buf")),
                contents: data,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
        }
    }
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        data: &[u8],
    ) {
        let mut view = belt.write_buffer(
            encoder,
            &self.buffer,
            0,
            unsafe {
                std::num::NonZeroU64::new_unchecked(std::mem::size_of_val(data) as u64)
            },
            device,
        );
        view.copy_from_slice(data);
    }

    pub fn bind_group_layout_entry(
        &self,
        binding: u32,
        read_only: bool,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX
                | wgpu::ShaderStages::FRAGMENT
                | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only },
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
