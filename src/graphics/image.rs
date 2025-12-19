use image::{ImageReader, RgbImage};

use crate::core::View;
use crate::graphics::Canvas;

/// An image that can be displayed as a View.
/// Supports any format the `image` crate handles (PNG, JPEG, PPM, etc.).
pub struct Image {
    width: u32,
    height: u32,
    data: RgbImage,
}

impl Image {
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

    /// Create an image from raw RGB data.
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

impl View for Image {
    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn draw(&self, canvas: &mut Canvas) {
        for (x, y, pixel) in self.data.enumerate_pixels() {
            let [r, g, b] = pixel.0;
            canvas.set_pixel_rgb(x, y, r, g, b);
        }
    }
}
