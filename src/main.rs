use curvy::{
    run, App, Button, Container, ImageWidget, Rect, RunConfig, UiTree, View, Widget,
    WidgetEvent,
};
use winit::event::WindowEvent;

struct DemoApp {
    tree: UiTree,
}

impl DemoApp {
    fn new() -> Self {
        let mut tree = UiTree::new();

        // Create a container as root
        let root = tree.add(Container::new(640, 480).with_background(0x1a1a2e), None);
        tree.set_bounds(root, Rect::new(0, 0, 640, 480));

        // Add an image
        let image = ImageWidget::from_file("src/image.ppm").expect("Failed to load image");
        let (img_w, img_h) = image.preferred_size();
        let image_id = tree.add(image, Some(root));
        tree.set_bounds(image_id, Rect::new(20, 20, img_w, img_h));

        // Add a button
        let button = Button::new(100, 40)
            .with_color(0x4a4e69)
            .with_hover_color(0x9a8c98)
            .with_pressed_color(0x22223b);
        let button_id = tree.add(button, Some(root));
        tree.set_bounds(button_id, Rect::new(20, 420, 100, 40));

        Self { tree }
    }
}

impl App for DemoApp {
    fn view(&self) -> &dyn View {
        &self.tree
    }

    fn on_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let hit = self.tree.hit_test(position.x as i32, position.y as i32);
                self.tree.set_hovered(hit);
                true
            }
            WindowEvent::MouseInput { state, .. } => {
                match state {
                    winit::event::ElementState::Pressed => {
                        if let Some(hovered) = self.tree.hovered() {
                            self.tree.set_pressed(Some(hovered));
                        }
                    }
                    winit::event::ElementState::Released => {
                        if let Some(pressed_id) = self.tree.pressed() {
                            if let Some(node) = self.tree.get_mut(pressed_id) {
                                node.widget_mut().on_event(&WidgetEvent::Click);
                            }
                        }
                        self.tree.set_pressed(None);
                    }
                }
                true
            }
            _ => false,
        }
    }
}

fn main() {
    let app = DemoApp::new();

    run(
        app,
        RunConfig {
            width: 900,
            height: 1480,
            resizable: false,
        },
    );
}
