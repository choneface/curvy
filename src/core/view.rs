use crate::graphics::Canvas;

/// The core trait for anything that can be drawn.
/// Views form a retained-mode widget tree.
pub trait View {
    /// Returns the preferred size of this view as (width, height).
    fn size(&self) -> (u32, u32);

    /// Draw this view to the canvas.
    fn draw(&self, canvas: &mut Canvas);
}
