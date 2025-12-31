//! File picker widget.
//!
//! A composite widget that combines a directory picker with a scrollable
//! file list. Supports filtering by file extension or substring.

use std::any::Any;
use std::fs;
use std::path::PathBuf;

use image::RgbImage;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::{draw_text, Canvas, TextStyle};

/// An entry in the file list.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// File name (not full path).
    pub name: String,
    /// Full path to the file.
    pub path: PathBuf,
    /// Whether this is a directory.
    pub is_dir: bool,
}

/// A file picker widget with directory selection and filtered file list.
pub struct FilePicker {
    /// Widget dimensions.
    width: u32,
    height: u32,

    // Directory picker images
    picker_normal: RgbImage,
    picker_hover: RgbImage,
    picker_btn_normal: RgbImage,
    picker_btn_hover: RgbImage,

    // Scrollbar images
    track_image: RgbImage,
    thumb_image: RgbImage,

    // List item images
    item_normal: RgbImage,
    item_hover: RgbImage,
    item_selected: RgbImage,

    /// Currently selected directory.
    selected_dir: Option<PathBuf>,
    /// File entries in the selected directory.
    entries: Vec<FileEntry>,
    /// Optional filter (e.g., ".crix").
    filter: Option<String>,
    /// Currently hovered item index.
    hovered_index: Option<usize>,
    /// Currently selected item index.
    selected_index: Option<usize>,

    /// Scroll offset in pixels.
    scroll_y: f32,
    /// Height of each list item (from item image).
    item_height: u32,
    /// Height of the picker area.
    picker_height: u32,
    /// Width of the scrollbar.
    scrollbar_width: u32,

    /// Text color for file names.
    text_color: u32,
    /// Text color for directory names.
    dir_color: u32,
    /// Padding inside items.
    padding: u32,

    /// Store binding key.
    binding: Option<String>,
    /// Dialog title for directory picker.
    dialog_title: String,

    /// Whether mouse is over the picker button.
    picker_btn_hovered: bool,
    /// Whether mouse is over the picker area.
    picker_hovered: bool,
}

impl FilePicker {
    /// Create a new file picker.
    pub fn new(
        width: u32,
        height: u32,
        picker_normal: RgbImage,
        picker_hover: RgbImage,
        picker_btn_normal: RgbImage,
        picker_btn_hover: RgbImage,
        track_image: RgbImage,
        thumb_image: RgbImage,
        item_normal: RgbImage,
        item_hover: RgbImage,
        item_selected: RgbImage,
    ) -> Self {
        let picker_height = picker_normal.height();
        let scrollbar_width = track_image.width();
        let item_height = item_normal.height();

        Self {
            width,
            height,
            picker_normal,
            picker_hover,
            picker_btn_normal,
            picker_btn_hover,
            track_image,
            thumb_image,
            item_normal,
            item_hover,
            item_selected,
            selected_dir: None,
            entries: Vec::new(),
            filter: None,
            hovered_index: None,
            selected_index: None,
            scroll_y: 0.0,
            item_height,
            picker_height,
            scrollbar_width,
            text_color: 0xDDDDDD,
            dir_color: 0x88AAFF,
            padding: 8,
            binding: None,
            dialog_title: "Select Directory".to_string(),
            picker_btn_hovered: false,
            picker_hovered: false,
        }
    }

    /// Set the filter string (e.g., ".crix").
    pub fn with_filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// Set the text color.
    pub fn with_text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    /// Set the directory color.
    pub fn with_dir_color(mut self, color: u32) -> Self {
        self.dir_color = color;
        self
    }

    /// Set the padding.
    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the store binding.
    pub fn with_binding(mut self, binding: impl Into<String>) -> Self {
        self.binding = Some(binding.into());
        self
    }

    /// Set the dialog title.
    pub fn with_dialog_title(mut self, title: impl Into<String>) -> Self {
        self.dialog_title = title.into();
        self
    }

    /// Get the binding key.
    pub fn binding(&self) -> Option<&str> {
        self.binding.as_deref()
    }

    /// Get the selected file path (if any).
    pub fn selected_file(&self) -> Option<&PathBuf> {
        self.selected_index.and_then(|i| self.entries.get(i).map(|e| &e.path))
    }

    /// Get the selected directory.
    pub fn selected_dir(&self) -> Option<&PathBuf> {
        self.selected_dir.as_ref()
    }

    /// Set the directory and refresh the file list.
    pub fn set_directory(&mut self, path: PathBuf) {
        self.selected_dir = Some(path);
        self.refresh_entries();
        self.scroll_y = 0.0;
        self.selected_index = None;
        self.hovered_index = None;
    }

    /// Refresh the entries from the current directory.
    fn refresh_entries(&mut self) {
        self.entries.clear();

        let Some(dir) = &self.selected_dir else {
            return;
        };

        let Ok(read_dir) = fs::read_dir(dir) else {
            return;
        };

        let mut entries: Vec<FileEntry> = read_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                // Skip hidden files
                if name.starts_with('.') {
                    return None;
                }

                // Apply filter - applies to both files and directories
                if let Some(ref filter) = self.filter {
                    if !name.contains(filter) {
                        return None;
                    }
                }

                Some(FileEntry { name, path, is_dir })
            })
            .collect();

        // Sort: directories first, then alphabetically
        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        self.entries = entries;
    }

    /// Open the directory picker dialog.
    fn open_dialog(&mut self) {
        let dialog = rfd::FileDialog::new().set_title(&self.dialog_title);

        let dialog = if let Some(ref path) = self.selected_dir {
            if path.exists() {
                dialog.set_directory(path)
            } else {
                dialog
            }
        } else {
            dialog
        };

        if let Some(path) = dialog.pick_folder() {
            self.set_directory(path);
        }
    }

    /// Get the list area bounds (below picker, excluding scrollbar).
    fn list_area(&self, bounds: &Rect) -> Rect {
        Rect::new(
            bounds.x,
            bounds.y + self.picker_height as i32,
            self.width - self.scrollbar_width,
            self.height - self.picker_height,
        )
    }

    /// Get the total content height.
    fn content_height(&self) -> u32 {
        (self.entries.len() as u32) * self.item_height
    }

    /// Get the visible list height.
    fn list_height(&self) -> u32 {
        self.height - self.picker_height
    }

    /// Get the maximum scroll offset.
    fn max_scroll(&self) -> f32 {
        let content = self.content_height();
        let visible = self.list_height();
        if content > visible {
            (content - visible) as f32
        } else {
            0.0
        }
    }

    /// Scroll by a delta amount.
    fn scroll_by(&mut self, delta: f32) {
        self.scroll_y = (self.scroll_y - delta * 30.0).clamp(0.0, self.max_scroll());
    }

    /// Get the scroll ratio (0.0 to 1.0).
    fn scroll_ratio(&self) -> f32 {
        let max = self.max_scroll();
        if max > 0.0 {
            self.scroll_y / max
        } else {
            0.0
        }
    }

    /// Get the thumb Y position.
    fn thumb_y(&self, track_y: i32) -> i32 {
        let thumb_h = self.thumb_image.height();
        let track_h = self.list_height().saturating_sub(thumb_h);
        track_y + (track_h as f32 * self.scroll_ratio()) as i32
    }

    /// Draw an image at a position with clipping.
    fn draw_image(&self, canvas: &mut Canvas, image: &RgbImage, x: i32, y: i32, clip: Option<&Rect>) {
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

    /// Draw the directory picker area.
    fn draw_picker(&self, canvas: &mut Canvas, bounds: &Rect) {
        let picker_bounds = Rect::new(bounds.x, bounds.y, self.width, self.picker_height);

        // Draw picker background
        let bg = if self.picker_hovered {
            &self.picker_hover
        } else {
            &self.picker_normal
        };
        self.draw_image(canvas, bg, bounds.x, bounds.y, Some(&picker_bounds));

        // Draw picker button
        let btn_width = self.picker_btn_normal.width();
        let btn_x = bounds.x + (self.width - btn_width) as i32;
        let btn = if self.picker_btn_hovered {
            &self.picker_btn_hover
        } else {
            &self.picker_btn_normal
        };
        self.draw_image(canvas, btn, btn_x, bounds.y, Some(&picker_bounds));

        // Draw directory path text
        let text_x = bounds.x + self.padding as i32;
        let text_y = bounds.y + (self.picker_height / 2) as i32;
        let text = if let Some(ref dir) = self.selected_dir {
            dir.to_string_lossy().to_string()
        } else {
            "Select directory...".to_string()
        };

        // Truncate if needed
        let display_text = if text.len() > 60 {
            format!("...{}", &text[text.len() - 57..])
        } else {
            text
        };

        let style = TextStyle { color: self.text_color };
        let text_clip = Rect::new(
            text_x,
            bounds.y,
            self.width - btn_width - self.padding * 2,
            self.picker_height,
        );
        draw_text(canvas, text_x, text_y, Some(&text_clip), &display_text, style);
    }

    /// Draw the scrollbar.
    fn draw_scrollbar(&self, canvas: &mut Canvas, bounds: &Rect) {
        let track_x = bounds.x + (self.width - self.scrollbar_width) as i32;
        let track_y = bounds.y + self.picker_height as i32;
        let track_h = self.list_height();

        // Tile track image
        let img_h = self.track_image.height();
        let mut y = track_y;
        while y < track_y + track_h as i32 {
            let remaining = (track_y + track_h as i32 - y) as u32;
            let draw_h = remaining.min(img_h);

            for (ix, iy, pixel) in self.track_image.enumerate_pixels() {
                if iy >= draw_h {
                    continue;
                }
                let px = track_x + ix as i32;
                let py = y + iy as i32;
                if px >= 0 && py >= 0 {
                    let [r, g, b] = pixel.0;
                    canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
                }
            }
            y += img_h as i32;
        }

        // Draw thumb
        let thumb_y = self.thumb_y(track_y);
        self.draw_image(canvas, &self.thumb_image, track_x, thumb_y, None);
    }

    /// Draw the file list.
    fn draw_list(&self, canvas: &mut Canvas, bounds: &Rect) {
        let list_area = self.list_area(bounds);

        // Set clip for list area
        canvas.set_clip(Some(list_area));

        let list_y = bounds.y + self.picker_height as i32;
        let item_width = self.width - self.scrollbar_width;

        for (i, entry) in self.entries.iter().enumerate() {
            let item_y = list_y + (i as i32 * self.item_height as i32) - self.scroll_y as i32;

            // Skip if completely outside view
            if item_y + self.item_height as i32 <= list_y || item_y >= list_y + self.list_height() as i32 {
                continue;
            }

            // Choose item background
            let bg = if self.selected_index == Some(i) {
                &self.item_selected
            } else if self.hovered_index == Some(i) {
                &self.item_hover
            } else {
                &self.item_normal
            };

            // Draw item background (stretch horizontally if needed)
            let img_width = bg.width();
            let scale_x = item_width as f32 / img_width as f32;

            for (ix, iy, pixel) in bg.enumerate_pixels() {
                let px = bounds.x + (ix as f32 * scale_x) as i32;
                let py = item_y + iy as i32;
                if px >= list_area.x && px < list_area.right() && py >= list_area.y && py < list_area.bottom() {
                    let [r, g, b] = pixel.0;
                    canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
                }
            }

            // Draw file name
            let text_x = bounds.x + self.padding as i32;
            let text_y = item_y + (self.item_height / 2) as i32;

            let prefix = if entry.is_dir { "[DIR] " } else { "" };
            let display_name = format!("{}{}", prefix, entry.name);
            let color = if entry.is_dir { self.dir_color } else { self.text_color };

            let style = TextStyle { color };
            draw_text(canvas, text_x, text_y, Some(&list_area), &display_name, style);
        }

        // Clear clip
        canvas.set_clip(None);
    }
}

impl Widget for FilePicker {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        // Draw picker area
        self.draw_picker(canvas, bounds);

        // Draw scrollbar
        self.draw_scrollbar(canvas, bounds);

        // Draw file list
        self.draw_list(canvas, bounds);
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        match event {
            WidgetEvent::Click => {
                // Handle click on picker button or list item
                // For now, always open dialog on click
                // TODO: Track click position to differentiate
                self.open_dialog();
                true
            }
            WidgetEvent::MouseWheel { delta_y } => {
                if self.max_scroll() > 0.0 {
                    self.scroll_by(*delta_y);
                    true
                } else {
                    false
                }
            }
            WidgetEvent::MouseMove { x: _, y: _ } => {
                // TODO: Update hover states based on position
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
