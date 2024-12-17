mod encoder;
mod format;

pub use encoder::*;
pub use format::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TileState {
    Light,
    Dark,
    #[default]
    Unset,
}

pub struct TilesBuffer {
    buffer: Box<[TileState]>,
    pub dim_size: u32,
}

impl TilesBuffer {
    pub fn new(dim_size: u32) -> Self {
        Self {
            buffer: vec![TileState::Unset; dim_size as usize * dim_size as usize]
                .into_boxed_slice(),
            dim_size,
        }
    }

    pub fn put_tile(&mut self, x: u32, y: u32, state: TileState) {
        *self.buffer.get_mut(self.index(x, y)).unwrap() = state;
    }

    fn get_tile(&mut self, x: u32, y: u32) -> TileState {
        self.buffer[self.index(x, y)]
    }

    fn index(&self, x: u32, y: u32) -> usize {
        #[cfg(debug_assertions)]
        if !self.within_bounds(x as i32, y as i32) {
            panic!("{x} or {y} is out of bounds of {}", self.dim_size);
        }

        (x * self.dim_size + y) as usize
    }

    pub fn within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.dim_size as i32 && y < self.dim_size as i32
    }

    pub fn to_image(&self, scale: u32) -> image::GrayImage {
        image::GrayImage::from_fn(self.dim_size * scale, self.dim_size * scale, |x, y| {
            //
            match self.buffer[((x / scale) * self.dim_size + (y / scale)) as usize] {
                TileState::Light => image::Luma([255]),
                TileState::Dark => image::Luma([0]),
                TileState::Unset => image::Luma([125]),
            }
        })
    }
}
