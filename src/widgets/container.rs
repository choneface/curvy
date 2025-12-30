use std::any::Any;

use image::{ImageReader, RgbImage};

use crate::core::{Rect, Widget, WidgetState};
use crate::graphics::Canvas;

/// Background type for a container - either a solid color or an image.
enum Background {
    Color(u32),
    Image(RgbImage),
}

/// A simple container widget that can have a background color or image.
/// Children are managed by the UiTree, not the container itself.
pub struct Container {
    width: u32,
    height: u32,
    background: Option<Background>,
}

impl Container {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            background: None,
        }
    }

    pub fn with_background(mut self, color: u32) -> Self {
        self.background = Some(Background::Color(color));
        self
    }

    /// Create a container with an image background.
    /// The container's size will be set to the image dimensions.
    pub fn from_image(path: &str) -> Result<Self, image::ImageError> {
        let reader = ImageReader::open(path)?;
        let img = reader.decode()?;
        let rgb = img.to_rgb8();

        Ok(Self {
            width: rgb.width(),
            height: rgb.height(),
            background: Some(Background::Image(rgb)),
        })
    }

    /// Set an image as the background.
    /// This will also update the container's size to match the image.
    pub fn with_image(mut self, path: &str) -> Result<Self, image::ImageError> {
        let reader = ImageReader::open(path)?;
        let img = reader.decode()?;
        let rgb = img.to_rgb8();

        self.width = rgb.width();
        self.height = rgb.height();
        self.background = Some(Background::Image(rgb));
        Ok(self)
    }

    pub fn transparent(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            background: None,
        }
    }
}

impl Widget for Container {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        match &self.background {
            Some(Background::Color(color)) => {
                for y in 0..bounds.height {
                    for x in 0..bounds.width {
                        let px = bounds.x + x as i32;
                        let py = bounds.y + y as i32;
                        if px >= 0 && py >= 0 {
                            canvas.set_pixel(px as u32, py as u32, *color);
                        }
                    }
                }
            }
            Some(Background::Image(image)) => {
                for (ix, iy, pixel) in image.enumerate_pixels() {
                    let x = bounds.x + ix as i32;
                    let y = bounds.y + iy as i32;

                    if x >= bounds.x
                        && x < bounds.right()
                        && y >= bounds.y
                        && y < bounds.bottom()
                    {
                        if x >= 0 && y >= 0 {
                            let [r, g, b] = pixel.0;
                            canvas.set_pixel_rgb(x as u32, y as u32, r, g, b);
                        }
                    }
                }
            }
            None => {}
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
