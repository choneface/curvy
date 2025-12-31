use crate::core::Rect;

/// A drawing surface that Views render to.
/// Wraps a mutable pixel buffer with drawing primitives.
pub struct Canvas<'a> {
    buffer: &'a mut [u32],
    width: u32,
    height: u32,
    clip_rect: Option<Rect>,
}

impl<'a> Canvas<'a> {
    pub fn new(buffer: &'a mut [u32], width: u32, height: u32) -> Self {
        Self {
            buffer,
            width,
            height,
            clip_rect: None,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the clipping rectangle. Only pixels within this rect will be drawn.
    pub fn set_clip(&mut self, rect: Option<Rect>) {
        self.clip_rect = rect;
    }

    /// Get the current clipping rectangle.
    pub fn clip_rect(&self) -> Option<&Rect> {
        self.clip_rect.as_ref()
    }

    /// Check if a pixel is within the clip rect (if set).
    #[inline]
    fn is_clipped(&self, x: u32, y: u32) -> bool {
        if let Some(ref clip) = self.clip_rect {
            let px = x as i32;
            let py = y as i32;
            px < clip.x || px >= clip.right() || py < clip.y || py >= clip.bottom()
        } else {
            false
        }
    }

    /// Set a single pixel. Coordinates outside bounds or clip rect are ignored.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height && !self.is_clipped(x, y) {
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
                if !self.is_clipped(px, py) {
                    let index = (py * self.width + px) as usize;
                    self.buffer[index] = color;
                }
            }
        }
    }
}
