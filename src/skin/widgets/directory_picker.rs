//! Directory picker widget.
//!
//! A widget that displays a selected directory path and opens a native
//! directory picker dialog when clicked.

use std::any::Any;
use std::path::PathBuf;

use image::RgbImage;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::{draw_text, Canvas, TextStyle};

/// A directory picker widget with skinnable background and button.
pub struct DirectoryPicker {
    /// Background image for normal state.
    normal: RgbImage,
    /// Background image for hover state.
    hover: RgbImage,
    /// Button image (the "..." button on the right).
    button_normal: RgbImage,
    /// Button image for hover state.
    button_hover: RgbImage,
    /// Widget dimensions.
    width: u32,
    height: u32,
    /// Currently selected directory path.
    selected_path: Option<PathBuf>,
    /// Placeholder text when no directory is selected.
    placeholder: String,
    /// Text color for the path display.
    text_color: u32,
    /// Text color for placeholder.
    placeholder_color: u32,
    /// Padding from edges.
    padding: u32,
    /// Font size.
    font_size: Option<f32>,
    /// Store binding key.
    binding: Option<String>,
    /// Whether the value has changed (dirty flag for store sync).
    dirty: bool,
    /// Button width (derived from button image).
    button_width: u32,
    /// Whether mouse is over the button area.
    button_hovered: bool,
    /// Dialog title.
    dialog_title: String,
}

impl DirectoryPicker {
    /// Create a new directory picker.
    pub fn new(
        normal: RgbImage,
        hover: RgbImage,
        button_normal: RgbImage,
        button_hover: RgbImage,
    ) -> Self {
        let width = normal.width();
        let height = normal.height();
        let button_width = button_normal.width();

        Self {
            normal,
            hover,
            button_normal,
            button_hover,
            width,
            height,
            selected_path: None,
            placeholder: "Select directory...".to_string(),
            text_color: 0x000000,
            placeholder_color: 0x888888,
            padding: 8,
            font_size: None,
            binding: None,
            dirty: false,
            button_width,
            button_hovered: false,
            dialog_title: "Select Directory".to_string(),
        }
    }

    /// Set placeholder text.
    pub fn with_placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set text color.
    pub fn with_text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    /// Set placeholder color.
    pub fn with_placeholder_color(mut self, color: u32) -> Self {
        self.placeholder_color = color;
        self
    }

    /// Set padding.
    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set font size.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set store binding.
    pub fn with_binding(mut self, binding: impl Into<String>) -> Self {
        self.binding = Some(binding.into());
        self
    }

    /// Set dialog title.
    pub fn with_dialog_title(mut self, title: impl Into<String>) -> Self {
        self.dialog_title = title.into();
        self
    }

    /// Get the binding key.
    pub fn binding(&self) -> Option<&str> {
        self.binding.as_deref()
    }

    /// Get the selected path as a string.
    pub fn path_string(&self) -> String {
        self.selected_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Set the path from a string (for store sync).
    pub fn set_path(&mut self, path: &str) {
        if path.is_empty() {
            self.selected_path = None;
        } else {
            self.selected_path = Some(PathBuf::from(path));
        }
    }

    /// Check if the value has changed.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clear the dirty flag.
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Open the directory picker dialog.
    fn open_dialog(&mut self) {
        let dialog = rfd::FileDialog::new().set_title(&self.dialog_title);

        // Start from current selection if available
        let dialog = if let Some(ref path) = self.selected_path {
            if path.exists() {
                dialog.set_directory(path)
            } else {
                dialog
            }
        } else {
            dialog
        };

        if let Some(path) = dialog.pick_folder() {
            self.selected_path = Some(path);
            self.dirty = true;
        }
    }

    /// Draw an image at a position.
    fn draw_image(&self, canvas: &mut Canvas, image: &RgbImage, x: i32, y: i32, clip: &Rect) {
        for (ix, iy, pixel) in image.enumerate_pixels() {
            let px = x + ix as i32;
            let py = y + iy as i32;
            if px >= clip.x && px < clip.right() && py >= clip.y && py < clip.bottom() {
                if px >= 0 && py >= 0 {
                    let [r, g, b] = pixel.0;
                    canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
                }
            }
        }
    }
}

impl Widget for DirectoryPicker {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, state: WidgetState) {
        // Draw background
        let bg = if state.hovered || state.focused {
            &self.hover
        } else {
            &self.normal
        };
        self.draw_image(canvas, bg, bounds.x, bounds.y, bounds);

        // Draw button
        let button_x = bounds.x + (self.width - self.button_width) as i32;
        let button_img = if self.button_hovered {
            &self.button_hover
        } else {
            &self.button_normal
        };
        self.draw_image(canvas, button_img, button_x, bounds.y, bounds);

        // Draw text (path or placeholder)
        let text_x = bounds.x + self.padding as i32;
        let text_y = bounds.y + (self.height / 2) as i32;
        let text_width = self.width - self.button_width - self.padding * 2;

        // Clip text to available area
        let text_clip = Rect::new(
            text_x,
            bounds.y,
            text_width,
            self.height,
        );

        let (text, color) = if let Some(ref path) = self.selected_path {
            (path.to_string_lossy().to_string(), self.text_color)
        } else {
            (self.placeholder.clone(), self.placeholder_color)
        };

        // Truncate text if too long (simple truncation with ellipsis)
        let display_text = if text.len() > 40 {
            format!("...{}", &text[text.len() - 37..])
        } else {
            text
        };

        let style = TextStyle {
            color,
        };

        draw_text(canvas, text_x, text_y, Some(&text_clip), &display_text, style);
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        match event {
            WidgetEvent::Click => {
                self.open_dialog();
                true
            }
            WidgetEvent::MouseMove { x: _, y: _ } => {
                // Track if mouse is over button for hover effect
                // Note: we'd need bounds here, which we don't have
                // For now, always show button as potentially hoverable
                false
            }
            _ => false,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
