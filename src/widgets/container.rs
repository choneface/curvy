use crate::core::{Rect, Widget, WidgetState};
use crate::graphics::Canvas;

/// A simple container widget that can have a background color.
/// Children are managed by the UiTree, not the container itself.
pub struct Container {
    width: u32,
    height: u32,
    background: Option<u32>,
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
        self.background = Some(color);
        self
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
        if let Some(color) = self.background {
            for y in 0..bounds.height {
                for x in 0..bounds.width {
                    let px = bounds.x + x as i32;
                    let py = bounds.y + y as i32;
                    if px >= 0 && py >= 0 {
                        canvas.set_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
