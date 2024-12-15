use crate::Format;

pub struct Encoder {
    image: image::RgbImage,
    format: Format,
    dim_size: u32,
}

impl Encoder {
    pub fn new(format: Format) -> Self {
        println!("Size: {0}x{0}", format.version.dim_size());
        let size = format.image_size();
        Self {
            image: image::RgbImage::from_pixel(size, size, image::Rgb([255, 255, 255])),
            dim_size: format.version.dim_size(),
            format,
        }
    }

    pub fn encode_str(&mut self, data: &str) -> &image::RgbImage {
        self.put_finder_pattern(0, 0);
        self.put_finder_pattern(self.dim_size - 7, 0);
        self.put_finder_pattern(0, self.dim_size - 7);
        &self.image
    }

    fn put_finder_pattern(&mut self, start_x: u32, start_y: u32) {
        for x in 0..7 {
            for y in 0..7 {
                match (x, y) {
                    (1 | 5, 1..6) => continue,
                    (1..6, 1 | 5) => continue,
                    _ => (),
                };
                self.put_dark_module(start_x + x, start_y + y);
            }
        }
    }

    fn put_dark_module(&mut self, x: u32, y: u32) {
        let start_px = (x + self.format.empty_gap as u32) * self.format.module_size as u32;
        let start_py = (y + self.format.empty_gap as u32) * self.format.module_size as u32;
        for px in 0..self.format.module_size as u32 {
            for py in 0..self.format.module_size as u32 {
                self.image
                    .put_pixel(start_px + px, start_py + py, image::Rgb([0, 0, 0]));
            }
        }
    }
}
