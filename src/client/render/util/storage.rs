use std::marker::PhantomData;

use wgpu::util::DeviceExt;

pub struct Storage {
    buffer: wgpu::Buffer,
    old_len: usize,
}

impl Storage {
    pub fn init<T: Default + bytemuck::Pod>(device: &wgpu::Device, name: &str) -> Self {
        let def = [T::default()];
        let default = bytemuck::cast_slice(&def);
        Self {
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&(name.to_owned() + " Uniform Buf")),
                contents: default,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            }),
            old_len: default.len(),
        }
    }
    pub fn init_with(device: &wgpu::Device, name: &str, data: &[u8]) -> Self {
        Self {
            buffer: Self::init_buf(device, name, data),
            old_len: data.len(),
        }
    }
    pub fn init_buf(device: &wgpu::Device, name: &str, data: &[u8]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&(name.to_owned() + " Uniform Buf")),
            contents: data,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        })
    }
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        data: &[u8],
    ) -> bool {
        if data.len() != self.old_len {
            self.buffer = Self::init_buf(device, "too lazy", data);
            self.old_len = data.len();
            true
        } else {
            let mut view = belt.write_buffer(
                encoder,
                &self.buffer,
                0,
                unsafe { std::num::NonZeroU64::new_unchecked(std::mem::size_of_val(data) as u64) },
                device,
            );
            view.copy_from_slice(data);
            false
        }
    }

    pub fn bind_group_layout_entry(
        &self,
        binding: u32,
        read_only: bool,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
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
