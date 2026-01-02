//! Checkbox widget.
//!
//! A toggleable checkbox widget with checked and unchecked image states.
//! Supports an optional text label and store binding.

use std::any::Any;

use image::RgbImage;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::{draw_text_sized, line_height_sized, Canvas, TextStyle};

/// A checkbox widget with two states: checked and unchecked.
pub struct Checkbox {
    /// Image for unchecked state.
    unchecked: RgbImage,
    /// Image for checked state.
    checked: RgbImage,
    /// Widget dimensions.
    width: u32,
    height: u32,
    /// Current checked state.
    is_checked: bool,
    /// Optional label text.
    label: Option<String>,
    /// Label text color.
    text_color: u32,
    /// Font size for label.
    font_size: Option<f32>,
    /// Padding between checkbox and label.
    padding: u32,
    /// Store binding key.
    binding: Option<String>,
    /// Action to trigger when checkbox is toggled.
    action: Option<String>,
    /// Flag indicating the state was modified since last sync.
    dirty: bool,
}

impl Checkbox {
    /// Create a new checkbox with the given state images.
    pub fn new(unchecked: RgbImage, checked: RgbImage) -> Self {
        let width = unchecked.width();
        let height = unchecked.height();
        Self {
            unchecked,
            checked,
            width,
            height,
            is_checked: false,
            label: None,
            text_color: 0xDDDDDD,
            font_size: None,
            padding: 8,
            binding: None,
            action: None,
            dirty: false,
        }
    }

    /// Set the label text.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the text color.
    pub fn with_text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    /// Set the font size.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set the padding between checkbox and label.
    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the store binding key.
    pub fn with_binding(mut self, binding: impl Into<String>) -> Self {
        self.binding = Some(binding.into());
        self
    }

    /// Set the initial checked state.
    pub fn with_checked(mut self, checked: bool) -> Self {
        self.is_checked = checked;
        self
    }

    /// Set the action to trigger when toggled.
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Get the action name.
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    /// Get the binding key.
    pub fn binding(&self) -> Option<&str> {
        self.binding.as_deref()
    }

    /// Check if the state has been modified since last sync.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clear the dirty flag (call after syncing to store).
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Get the current checked state.
    pub fn is_checked(&self) -> bool {
        self.is_checked
    }

    /// Set the checked state.
    pub fn set_checked(&mut self, checked: bool) {
        if self.is_checked != checked {
            self.is_checked = checked;
            self.dirty = true;
        }
    }

    /// Toggle the checked state.
    pub fn toggle(&mut self) {
        self.is_checked = !self.is_checked;
        self.dirty = true;
    }

    /// Get the effective font size.
    fn effective_font_size(&self) -> f32 {
        self.font_size.unwrap_or(16.0)
    }

    fn draw_image(&self, canvas: &mut Canvas, x: i32, y: i32, image: &RgbImage, clip: Option<&Rect>) {
        for (ix, iy, pixel) in image.enumerate_pixels() {
            let px = x + ix as i32;
            let py = y + iy as i32;

            if let Some(clip) = clip {
                if px < clip.x || px >= clip.right() || py < clip.y || py >= clip.bottom() {
                    continue;
                }
            }

            if px >= 0 && py >= 0 {
                let [r, g, b] = pixel.0;
                canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
            }
        }
    }
}

impl Widget for Checkbox {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        // Choose image based on checked state
        let image = if self.is_checked {
            &self.checked
        } else {
            &self.unchecked
        };

        // Center the checkbox image vertically
        let img_height = image.height();
        let y_offset = (bounds.height.saturating_sub(img_height)) / 2;
        let img_y = bounds.y + y_offset as i32;

        // Draw the checkbox image
        self.draw_image(canvas, bounds.x, img_y, image, Some(bounds));

        // Draw label if present
        if let Some(ref label) = self.label {
            let font_size = self.effective_font_size();
            let text_height = line_height_sized(font_size);

            // Position label to the right of the checkbox
            let label_x = bounds.x + image.width() as i32 + self.padding as i32;
            let label_y = bounds.y + (bounds.height as i32 - text_height as i32) / 2;

            let style = TextStyle::with_color(self.text_color);
            draw_text_sized(canvas, label_x, label_y, Some(bounds), label, style, font_size);
        }
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        if let WidgetEvent::Click = event {
            self.toggle();
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
