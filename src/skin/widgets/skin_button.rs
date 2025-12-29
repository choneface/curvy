use image::RgbImage;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::Canvas;

/// A button widget driven by skin assets for each state.
pub struct SkinButton {
    normal: RgbImage,
    hover: RgbImage,
    pressed: RgbImage,
    width: u32,
    height: u32,
    action: Option<String>,
}

impl SkinButton {
    /// Create a skin button with images for each state.
    pub fn new(
        normal: RgbImage,
        hover: RgbImage,
        pressed: RgbImage,
        action: Option<String>,
    ) -> Self {
        let width = normal.width();
        let height = normal.height();
        Self {
            normal,
            hover,
            pressed,
            width,
            height,
            action,
        }
    }

    /// Get the action string for this button.
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    fn draw_image(&self, canvas: &mut Canvas, bounds: &Rect, image: &RgbImage) {
        for (ix, iy, pixel) in image.enumerate_pixels() {
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
}

impl Widget for SkinButton {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, state: WidgetState) {
        let image = if state.pressed {
            &self.pressed
        } else if state.hovered {
            &self.hover
        } else {
            &self.normal
        };

        self.draw_image(canvas, bounds, image);
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        if let WidgetEvent::Click = event {
            if let Some(action) = &self.action {
                println!("Button action: {}", action);
            }
            return true;
        }
        false
    }
}
