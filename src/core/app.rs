use winit::event::WindowEvent;

use crate::core::View;

/// Trait for applications using the Curvy framework.
pub trait App {
    /// Returns the root view to render.
    fn view(&self) -> &dyn View;

    /// Handle a window event. Return true if the view needs to be redrawn.
    fn on_event(&mut self, event: &WindowEvent) -> bool {
        let _ = event;
        false
    }
}

/// A simple app runner that wraps a View without event handling.
pub struct AppRunner<V: View> {
    view: V,
}

impl<V: View> AppRunner<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }
}

impl<V: View> App for AppRunner<V> {
    fn view(&self) -> &dyn View {
        &self.view
    }
}
