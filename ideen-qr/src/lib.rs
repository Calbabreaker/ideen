mod encoder;
mod format;

pub use encoder::*;
pub use format::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ModuleState {
    Light,
    Dark,
    #[default]
    Unset,
}

pub struct ImageModuleBuffer {
    buffer: Box<[ModuleState]>,
    pub dim_size: u32,
}

impl ImageModuleBuffer {
    pub fn new(dim_size: u32) -> Self {
        Self {
            buffer: vec![ModuleState::Unset; dim_size as usize * dim_size as usize]
                .into_boxed_slice(),
            dim_size,
        }
    }

    pub fn set_module(&mut self, x: u32, y: u32, state: ModuleState) {
        #[cfg(debug_assertions)]
        if x >= self.dim_size || y >= self.dim_size {
            panic!("{x} or {y} is out of bounds of {}", self.dim_size);
        }

        let index = x * self.dim_size + y;
        *self.buffer.get_mut(index as usize).unwrap() = state;
    }

    pub fn to_image(&self, scale: u32) -> image::GrayImage {
        image::GrayImage::from_fn(self.dim_size * scale, self.dim_size * scale, |x, y| {
            //
            match self.buffer[((x / scale) * self.dim_size + (y / scale)) as usize] {
                ModuleState::Light => image::Luma([255]),
                ModuleState::Dark => image::Luma([0]),
                ModuleState::Unset => image::Luma([125]),
            }
        })
    }
}
