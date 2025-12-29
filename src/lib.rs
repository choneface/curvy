pub mod core;
pub mod graphics;
pub mod platform;
pub mod skin;
pub mod widgets;

// Re-export commonly used types at the crate root
pub use core::{App, AppRunner, Node, NodeId, Rect, UiTree, View, Widget, WidgetEvent, WidgetState};
pub use graphics::{Canvas, Image};
pub use platform::{run, RunConfig};
pub use skin::{LoadedSkin, SkinBuilder, SkinError, SkinWindow};
pub use widgets::{Button, Container, ImageWidget};
