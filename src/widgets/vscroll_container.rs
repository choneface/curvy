//! Vertical scroll container widget.
//!
//! A container that holds a single child and provides vertical scrolling
//! when the child's height exceeds the container's viewport.

use std::any::Any;

use crate::core::{Rect, Widget, WidgetEvent, WidgetState};
use crate::graphics::Canvas;

/// A vertical scroll container that clips and scrolls its child content.
pub struct VScrollContainer {
    /// Container dimensions.
    width: u32,
    height: u32,
    /// Width of the scrollbar area.
    scrollbar_width: u32,
    /// Current scroll offset in pixels.
    scroll_y: f32,
    /// Total height of the content (child height).
    content_height: u32,
    /// The child widget.
    child: Option<Box<dyn Widget>>,
    /// Scroll speed multiplier.
    scroll_speed: f32,
}

impl VScrollContainer {
    /// Create a new vertical scroll container.
    pub fn new(width: u32, height: u32, scrollbar_width: u32) -> Self {
        Self {
            width,
            height,
            scrollbar_width,
            scroll_y: 0.0,
            content_height: 0,
            child: None,
            scroll_speed: 1.0,
        }
    }

    /// Set the child widget.
    pub fn set_child(&mut self, child: Box<dyn Widget>) {
        let (_, h) = child.preferred_size();
        self.content_height = h;
        self.child = Some(child);
        // Clamp scroll position
        self.scroll_y = self.scroll_y.clamp(0.0, self.max_scroll());
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

    /// Calculate the thumb height based on content/viewport ratio.
    pub fn thumb_height(&self) -> u32 {
        if self.content_height == 0 {
            return self.height;
        }
        let ratio = self.height as f32 / self.content_height as f32;
        let thumb_h = (self.height as f32 * ratio).max(20.0); // Minimum 20px thumb
        thumb_h.min(self.height as f32) as u32
    }

    /// Calculate the thumb Y position within the scrollbar track.
    pub fn thumb_y(&self, track_y: i32) -> i32 {
        let track_height = self.height - self.thumb_height();
        track_y + (track_height as f32 * self.scroll_ratio()) as i32
    }

    /// Get the scrollbar track rect (for drawing).
    pub fn track_rect(&self, bounds: &Rect) -> Rect {
        Rect::new(
            bounds.x + self.viewport_width() as i32,
            bounds.y,
            self.scrollbar_width,
            self.height,
        )
    }

    /// Get the scrollbar thumb rect (for drawing).
    pub fn thumb_rect(&self, bounds: &Rect) -> Rect {
        let track = self.track_rect(bounds);
        Rect::new(
            track.x,
            self.thumb_y(track.y),
            self.scrollbar_width,
            self.thumb_height(),
        )
    }
}

impl Widget for VScrollContainer {
    fn draw(&self, canvas: &mut Canvas, bounds: &Rect, state: WidgetState) {
        // Draw scrollbar track (simple gray for now - will be overridden by SkinVScroll)
        let track = self.track_rect(bounds);
        canvas.fill_rect(
            track.x as u32,
            track.y as u32,
            track.width,
            track.height,
            0x333333,
        );

        // Draw scrollbar thumb
        let thumb = self.thumb_rect(bounds);
        let thumb_color = if state.pressed {
            0xAAAAAA
        } else if state.hovered {
            0x888888
        } else {
            0x666666
        };
        canvas.fill_rect(
            thumb.x as u32,
            thumb.y as u32,
            thumb.width,
            thumb.height,
            thumb_color,
        );

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
