use image::{ImageReader, RgbImage};

use crate::core::{Rect, Widget, WidgetState};
use crate::graphics::Canvas;

/// An image widget that displays a loaded image.
pub struct ImageWidget {
    width: u32,
    height: u32,
    data: RgbImage,
}

impl ImageWidget {
    /// Load an image from a file path.
    pub fn from_file(path: &str) -> Result<Self, image::ImageError> {
        let reader = ImageReader::open(path)?;
        let img = reader.decode()?;
        let rgb = img.to_rgb8();

        Ok(Self {
            width: rgb.width(),
            height: rgb.height(),
            data: rgb,
        })
    }

    /// Create an image widget from raw RGB data.
    pub fn from_rgb(data: RgbImage) -> Self {
        Self {
            width: data.width(),
            height: data.height(),
            data,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Widget for ImageWidget {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        for (ix, iy, pixel) in self.data.enumerate_pixels() {
            let x = bounds.x + ix as i32;
            let y = bounds.y + iy as i32;

            // Clip to bounds
            if x >= bounds.x && x < bounds.right() && y >= bounds.y && y < bounds.bottom() {
                if x >= 0 && y >= 0 {
                    let [r, g, b] = pixel.0;
                    canvas.set_pixel_rgb(x as u32, y as u32, r, g, b);
                }
            }
        }
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
