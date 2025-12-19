use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::event_loop::OwnedDisplayHandle;
use winit::window::Window;

use crate::core::View;
use crate::graphics::Canvas;

/// Handles rendering Views to the window surface.
pub struct Renderer {
    surface: Surface<OwnedDisplayHandle, Rc<Window>>,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(context: &softbuffer::Context<OwnedDisplayHandle>, window: Rc<Window>) -> Self {
        let size = window.inner_size();
        let surface = Surface::new(context, window).expect("Failed to create surface");

        Self {
            surface,
            width: size.width,
            height: size.height,
        }
    }

    /// Resize the rendering surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface.resize(w, h).expect("Failed to resize surface");
        }
    }

    /// Render a View to the window.
    pub fn render(&mut self, view: &dyn View) {
        let mut buffer = self.surface.buffer_mut().expect("Failed to get buffer");

        {
            let mut canvas = Canvas::new(&mut buffer, self.width, self.height);
            canvas.clear(0x000000); // Clear to black
            view.draw(&mut canvas);
        }

        buffer.present().expect("Failed to present buffer");
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
