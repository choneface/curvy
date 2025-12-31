//! Skinned vertical scroll container widget.
//!
//! Uses images for the scrollbar track and thumb instead of solid colors.

use std::any::Any;

use image::RgbImage;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::Canvas;

/// A skinned vertical scroll container with image-based scrollbar.
pub struct SkinVScroll {
    /// Container dimensions.
    width: u32,
    height: u32,
    /// Width of the scrollbar (from track image).
    scrollbar_width: u32,
    /// Current scroll offset in pixels.
    scroll_y: f32,
    /// Total height of the content (child height).
    content_height: u32,
    /// The child widget.
    child: Option<Box<dyn Widget>>,
    /// Scroll speed multiplier.
    scroll_speed: f32,
    /// Track image (tiled or stretched vertically).
    track_image: RgbImage,
    /// Thumb image.
    thumb_image: RgbImage,
}

impl SkinVScroll {
    /// Create a new skinned vertical scroll container.
    pub fn new(
        width: u32,
        height: u32,
        track_image: RgbImage,
        thumb_image: RgbImage,
    ) -> Self {
        let scrollbar_width = track_image.width();
        Self {
            width,
            height,
            scrollbar_width,
            scroll_y: 0.0,
            content_height: 0,
            child: None,
            scroll_speed: 1.0,
            track_image,
            thumb_image,
        }
    }

    /// Set the child widget.
    pub fn set_child(&mut self, child: Box<dyn Widget>) {
        let (_, h) = child.preferred_size();
        self.content_height = h;
        self.child = Some(child);
        self.scroll_y = self.scroll_y.clamp(0.0, self.max_scroll());
    }

    /// Builder method to set the child.
    pub fn with_child(mut self, child: Box<dyn Widget>) -> Self {
        self.set_child(child);
        self
    }

    /// Set the content height manually (useful when child doesn't report size).
    pub fn with_content_height(mut self, height: u32) -> Self {
        self.content_height = height;
        self
    }

    /// Set the scroll speed multiplier.
    pub fn with_scroll_speed(mut self, speed: f32) -> Self {
        self.scroll_speed = speed;
        self
    }

    /// Get the viewport width (container width minus scrollbar).
    pub fn viewport_width(&self) -> u32 {
        self.width.saturating_sub(self.scrollbar_width)
    }

    /// Get the viewport height.
    pub fn viewport_height(&self) -> u32 {
        self.height
    }

    /// Get the maximum scroll offset.
    pub fn max_scroll(&self) -> f32 {
        if self.content_height > self.height {
            (self.content_height - self.height) as f32
        } else {
            0.0
        }
    }

    /// Scroll by a delta amount.
    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_y = (self.scroll_y - delta * self.scroll_speed).clamp(0.0, self.max_scroll());
    }

    /// Get the current scroll position as a ratio (0.0 to 1.0).
    pub fn scroll_ratio(&self) -> f32 {
        let max = self.max_scroll();
        if max > 0.0 {
            self.scroll_y / max
        } else {
            0.0
        }
    }

    /// Get the thumb height (uses actual image height).
    pub fn thumb_height(&self) -> u32 {
        self.thumb_image.height()
    }

    /// Calculate the thumb Y position within the scrollbar track.
    pub fn thumb_y(&self, track_y: i32) -> i32 {
        let track_height = self.height.saturating_sub(self.thumb_height());
        track_y + (track_height as f32 * self.scroll_ratio()) as i32
    }

    /// Draw an image at a position, respecting canvas clipping.
    fn draw_image(&self, canvas: &mut Canvas, image: &RgbImage, x: i32, y: i32) {
        for (ix, iy, pixel) in image.enumerate_pixels() {
            let px = x + ix as i32;
            let py = y + iy as i32;
            if px >= 0 && py >= 0 {
                let [r, g, b] = pixel.0;
                canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
            }
        }
    }

    /// Draw the track image, tiling vertically if needed.
    fn draw_track(&self, canvas: &mut Canvas, bounds: &Rect) {
        let track_x = bounds.x + self.viewport_width() as i32;
        let track_height = self.track_image.height();

        // Tile the track image vertically
        let mut y = bounds.y;
        while y < bounds.y + self.height as i32 {
            // Clip the last tile if it extends beyond bounds
            let remaining = (bounds.y + self.height as i32 - y) as u32;
            let draw_height = remaining.min(track_height);

            for (ix, iy, pixel) in self.track_image.enumerate_pixels() {
                if iy >= draw_height {
                    continue;
                }
                let px = track_x + ix as i32;
                let py = y + iy as i32;
                if px >= 0 && py >= 0 {
                    let [r, g, b] = pixel.0;
                    canvas.set_pixel_rgb(px as u32, py as u32, r, g, b);
                }
            }
            y += track_height as i32;
        }
    }

    /// Draw the thumb image at the correct scroll position.
    fn draw_thumb(&self, canvas: &mut Canvas, bounds: &Rect) {
        let track_x = bounds.x + self.viewport_width() as i32;
        let thumb_y = self.thumb_y(bounds.y);
        self.draw_image(canvas, &self.thumb_image, track_x, thumb_y);
    }
}

impl Widget for SkinVScroll {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, _state: WidgetState) {
        // Draw scrollbar track
        self.draw_track(canvas, bounds);

        // Draw scrollbar thumb
        self.draw_thumb(canvas, bounds);

        // Set clip rect for viewport
        let viewport = Rect::new(
            bounds.x,
            bounds.y,
            self.viewport_width(),
            self.viewport_height(),
        );
        canvas.set_clip(Some(viewport));

        // Draw child with scroll offset
        if let Some(ref child) = self.child {
            let child_bounds = Rect::new(
                bounds.x,
                bounds.y - self.scroll_y as i32,
                self.viewport_width(),
                self.content_height,
            );
            child.draw(canvas, &child_bounds, WidgetState::default());
        }

        // Clear clip
        canvas.set_clip(None);
    }

    fn preferred_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn on_event(&mut self, event: &WidgetEvent) -> bool {
        match event {
            WidgetEvent::MouseWheel { delta_y } => {
                if self.max_scroll() > 0.0 {
                    self.scroll_by(*delta_y);
                    true
                } else {
                    false
                }
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
