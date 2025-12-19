/// A drawing surface that Views render to.
/// Wraps a mutable pixel buffer with drawing primitives.
pub struct Canvas<'a> {
    buffer: &'a mut [u32],
    width: u32,
    height: u32,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u32], width: u32, height: u32) -> Self {
        Self { buffer, width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set a single pixel. Coordinates outside bounds are ignored.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.buffer[index] = color;
        }
    }

    /// Set a pixel using RGB components.
    pub fn set_pixel_rgb(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        self.set_pixel(x, y, color);
    }

    /// Fill the entire canvas with a color.
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    /// Fill a rectangular region.
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);

        for py in y..y_end {
            for px in x..x_end {
                let index = (py * self.width + px) as usize;
                self.buffer[index] = color;
            }
        }
    }
}
