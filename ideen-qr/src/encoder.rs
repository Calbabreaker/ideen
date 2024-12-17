use crate::{Encoding, ErrorLevel, Format, TileState, TilesBuffer};

const FINDER_PATTERN: [&str; 9] = [
    ".........", //
    ".#######.", //
    ".#.....#.", //
    ".#.###.#.", //
    ".#.###.#.", //
    ".#.###.#.", //
    ".#.....#.", //
    ".#######.", //
    ".........", //
];

const ALIGNMENT_PATTERN: [&str; 5] = [
    "#####", //
    "#...#", //
    "#.#.#", //
    "#...#", //
    "#####", //
];

pub struct Encoder {
    buffer: TilesBuffer,
    format: Format,
}

impl Encoder {
    pub fn new(format: Format) -> Self {
        let dim_size = format.version.dim_size();
        Self {
            buffer: TilesBuffer::new(dim_size),
            format,
        }
    }

    pub fn set_encoding(&mut self, encoding: Encoding) {
        self.format.encoding = encoding;
    }

    pub fn set_error_level(&mut self, error_level: ErrorLevel) {
        self.format.error_level = error_level;
    }

    pub fn encode_str(&mut self, data: &str) -> &TilesBuffer {
        let dim_size = self.buffer.dim_size;
        self.put_pattern(-1, -1, &FINDER_PATTERN);
        self.put_pattern(dim_size as i32 - 8, -1, &FINDER_PATTERN);
        self.put_pattern(-1, dim_size as i32 - 8, &FINDER_PATTERN);

        for x in (4..dim_size).step_by(20) {
            for y in (4..dim_size).step_by(20) {
                if self.buffer.get_tile(x + 2, y + 2) == TileState::Unset {
                    self.put_pattern(x as i32, y as i32, &ALIGNMENT_PATTERN);
                }
            }
        }

        self.put_timing_pattern(6, 7, dim_size - 7, true);
        self.put_timing_pattern(6, 7, dim_size - 7, false);

        &self.buffer
    }

    pub fn put_timing_pattern(&mut self, a: u32, start: u32, end: u32, flip: bool) {
        for i in start..end {
            let state = if i % 2 == 0 {
                TileState::Dark
            } else {
                TileState::Light
            };
            if flip {
                self.buffer.put_tile(i, a, state);
            } else {
                self.buffer.put_tile(a, i, state);
            }
        }
    }

    fn put_pattern(&mut self, start_x: i32, start_y: i32, pattern: &[&str]) {
        for (x, str) in pattern.iter().enumerate() {
            for (y, char) in str.chars().enumerate() {
                let pos_x = start_x + x as i32;
                let pos_y = start_y + y as i32;
                let state = if char == '#' {
                    TileState::Dark
                } else {
                    TileState::Light
                };
                if self.buffer.within_bounds(pos_x, pos_y) {
                    self.buffer.put_tile(pos_x as u32, pos_y as u32, state);
                }
            }
        }
    }
}
