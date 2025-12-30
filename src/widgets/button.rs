use std::any::Any;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::Canvas;

/// A simple button widget with customizable colors.
pub struct Button {
    width: u32,
    height: u32,
    color: u32,
    hover_color: u32,
    pressed_color: u32,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color: 0x444444,
            hover_color: 0x666666,
            pressed_color: 0x222222,
            on_click: None,
        }
    }

    pub fn with_color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    pub fn with_hover_color(mut self, color: u32) -> Self {
        self.hover_color = color;
        self
    }

    pub fn with_pressed_color(mut self, color: u32) -> Self {
        self.pressed_color = color;
        self
    }

    pub fn on_click(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl Widget for Button {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, state: WidgetState) {
        let color = if state.pressed {
            self.pressed_color
        } else if state.hovered {
            self.hover_color
        } else {
            self.color
        };

        // Draw filled rectangle
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

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        if let WidgetEvent::Click = event {
            if let Some(ref mut callback) = self.on_click {
                callback();
            }
            return true;
        }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
