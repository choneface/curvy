pub mod core;
pub mod graphics;
pub mod platform;

// Re-export commonly used types at the crate root
pub use core::{App, AppRunner, View};
pub use graphics::{Canvas, Image};
pub use platform::{run, RunConfig};
