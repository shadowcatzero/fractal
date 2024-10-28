use nalgebra::Vector2;

#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct View {
    pub scale: Vector2<f32>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            scale: Vector2::zeros(),
        }
    }
}

impl View {
    pub fn new(size: &Vector2<u32>) -> Self {
        let fsize: Vector2<f32> = size.cast();
        let scale = if size.x < size.y {
            Vector2::new(fsize.x / fsize.y, 1.0)
        } else {
            Vector2::new(1.0, fsize.y / fsize.x)
        };
        View { scale }
    }
}

unsafe impl bytemuck::Pod for View {}
unsafe impl bytemuck::Zeroable for View {}
