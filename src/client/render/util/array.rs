use wgpu::{util::DeviceExt, BufferAddress, BufferUsages};

pub struct ArrayBuffer<T: bytemuck::Pod> {
    len: usize,
    new_len: usize,
    buffer: wgpu::Buffer,
    label: String,
    usage: BufferUsages,
    update: Option<Vec<T>>,
}

impl<T: bytemuck::Pod> ArrayBuffer<T> {
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
    ) -> bool {
        let mut resized = false;
        if self.new_len != self.len {
            let new = Self::init_buf(device, &self.label, self.new_len, self.usage);
            let cpy_len = self.len.min(self.new_len);
            if cpy_len != 0 {
                encoder.copy_buffer_to_buffer(
                    &self.buffer,
                    0,
                    &new,
                    0,
                    (cpy_len * std::mem::size_of::<T>()) as u64,
                );
            }
            self.len = self.new_len;
            resized = true;
            self.buffer = new;
        }
        if self.len == 0 {
            return resized;
        }
        if let Some(update) = self.update.take() {
            let mut view = belt.write_buffer(
                encoder,
                &self.buffer,
                0,
                unsafe {
                    std::num::NonZeroU64::new_unchecked(std::mem::size_of_val(&update[..]) as u64)
                },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(&update));
        }
        resized
    }

    pub fn set(&mut self, data: Vec<T>) {
        self.new_len = data.len();
        self.update = Some(data);
    }

    pub fn init(device: &wgpu::Device, label: &str, usage: BufferUsages) -> Self {
        let label = &(label.to_owned() + " Buffer");
        Self::init_with(device, label, usage, &[])
    }

    pub fn init_with(device: &wgpu::Device, label: &str, usage: BufferUsages, data: &[T]) -> Self {
        let label = &(label.to_owned() + " Buffer");
        Self {
            len: data.len(),
            new_len: data.len(),
            buffer: if data.is_empty() {
                Self::init_buf(device, label, 0, usage)
            } else {
                Self::init_buf_with(device, label, usage, data)
            },
            label: label.to_string(),
            update: None,
            usage,
        }
    }

    fn init_buf(
        device: &wgpu::Device,
        label: &str,
        size: usize,
        usage: BufferUsages,
    ) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            usage: usage | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            size: (size.max(1) * std::mem::size_of::<T>()) as u64,
            mapped_at_creation: false,
        })
    }

    fn init_buf_with(
        device: &wgpu::Device,
        label: &str,
        usage: BufferUsages,
        data: &[T],
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            usage: usage | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            contents: bytemuck::cast_slice(data),
        })
    }

    pub fn bind_group_layout_entry(
        &self,
        binding: u32,
        ty: wgpu::BufferBindingType,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty,
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

pub struct ArrBufUpdate<T> {
    pub offset: usize,
    pub data: Vec<T>,
}
