use std::any::Any;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::{
    caret_x_sized, draw_text_sized, line_height_sized, Canvas, TextStyle,
};
use crate::skin::types::{TextAlign, VerticalAlign};

/// A static text widget for displaying non-editable text.
/// Can be bound to a Store key to display dynamic values.
pub struct StaticText {
    /// The text content to display.
    content: String,
    /// Font size in pixels.
    font_size: f32,
    /// Text color.
    text_color: u32,
    /// Horizontal alignment.
    text_align: TextAlign,
    /// Vertical alignment.
    vertical_align: VerticalAlign,
    /// Padding from edges.
    padding: u32,
    /// Store binding key for reading values.
    binding: Option<String>,
}

impl StaticText {
    /// Create a new static text widget.
    pub fn new(content: String) -> Self {
        Self {
            content,
            font_size: 16.0,
            text_color: 0x000000, // Black
            text_align: TextAlign::Left,
            vertical_align: VerticalAlign::Center,
            padding: 0,
            binding: None,
        }
    }

    /// Set the font size.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the text color.
    pub fn with_text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    /// Set the horizontal alignment.
    pub fn with_text_align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    /// Set the vertical alignment.
    pub fn with_vertical_align(mut self, align: VerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }

    /// Set the padding.
    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the store binding key.
    pub fn with_binding(mut self, binding: String) -> Self {
        self.binding = Some(binding);
        self
    }

    /// Get the binding key.
    pub fn binding(&self) -> Option<&str> {
        self.binding.as_deref()
    }

    /// Get the text content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set the text content.
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Measure the width of the text.
    fn text_width(&self) -> u32 {
        caret_x_sized(&self.content, self.content.chars().count(), self.font_size)
    }
}

impl Widget for StaticText {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        // Calculate content rect (with padding)
        let content_rect = Rect::new(
            bounds.x + self.padding as i32,
            bounds.y + self.padding as i32,
            bounds.width.saturating_sub(self.padding * 2),
            bounds.height.saturating_sub(self.padding * 2),
        );

        let text_height = line_height_sized(self.font_size);
        let text_width = self.text_width();

        // Calculate x position based on horizontal alignment
        let text_x = match self.text_align {
            TextAlign::Left => content_rect.x,
            TextAlign::Center => {
                content_rect.x + (content_rect.width as i32 - text_width as i32) / 2
            }
            TextAlign::Right => {
                content_rect.x + content_rect.width as i32 - text_width as i32
            }
        };

        // Calculate y position based on vertical alignment
        let text_y = match self.vertical_align {
            VerticalAlign::Top => content_rect.y,
            VerticalAlign::Center => {
                content_rect.y + (content_rect.height as i32 - text_height as i32) / 2
            }
            VerticalAlign::Bottom => {
                content_rect.y + content_rect.height as i32 - text_height as i32
            }
        };

        // Draw text clipped to content rect
        draw_text_sized(
            canvas,
            text_x,
            text_y,
            Some(&content_rect),
            &self.content,
            TextStyle::with_color(self.text_color),
            self.font_size,
        );
    }

    fn preferred_size(&self) -> (u32, u32) {
        let width = self.text_width() + self.padding * 2;
        let height = line_height_sized(self.font_size) + self.padding * 2;
        (width, height)
    }

    fn on_event(&mut self, _event: &WidgetEvent) -> bool {
        false // Static text doesn't handle events
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
