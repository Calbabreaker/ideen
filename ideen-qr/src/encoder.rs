use crate::{Encoding, ErrorLevel, Format, ImageModuleBuffer, ModuleState};

pub struct Encoder {
    buffer: ImageModuleBuffer,
    format: Format,
}

impl Encoder {
    pub fn new(format: Format) -> Self {
        let dim_size = format.version.dim_size();
        Self {
            buffer: ImageModuleBuffer::new(dim_size),
            format,
        }
    }

    pub fn set_encoding(&mut self, encoding: Encoding) {
        self.format.encoding = encoding;
    }

    pub fn set_error_level(&mut self, error_level: ErrorLevel) {
        self.format.error_level = error_level;
    }

    pub fn encode_str(&mut self, data: &str) -> &ImageModuleBuffer {
        self.put_finder_pattern(0, 0);
        self.put_finder_pattern(self.buffer.dim_size - 7, 0);
        self.put_finder_pattern(0, self.buffer.dim_size - 7);
        &self.buffer
    }

    fn put_finder_pattern(&mut self, start_x: u32, start_y: u32) {
        for x in 0..7 {
            for y in 0..7 {
                let state = match (x, y) {
                    (1 | 5, 1..6) | (1..6, 1 | 5) => ModuleState::Light,
                    _ => ModuleState::Dark,
                };
                self.buffer.set_module(start_x + x, start_y + y, state);
            }
        }
    }
}
