use std::any::Any;

use image::RgbImage;

use crate::core::{Rect, Widget, WidgetState};
use crate::graphics::Canvas;

/// A static image widget driven by a skin asset.
pub struct SkinImage {
    image: RgbImage,
    width: u32,
    height: u32,
}

impl SkinImage {
    /// Create a skin image widget from loaded RGB image data.
    pub fn new(image: RgbImage) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            image,
            width,
            height,
        }
    }
}

impl Widget for SkinImage {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        for (ix, iy, pixel) in self.image.enumerate_pixels() {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
