use std::path::PathBuf;
use std::process::Command;

use clap::{Parser, Subcommand};
use crix::{
    run, init_font, Action, ActionDispatcher, App, AppBundle, KeyCode,
    LuaActionHandler, RunConfig, Services, SkinBuilder, StaticText,
    Store, TextInput, UiTree, View, WidgetEvent,
    skin::widgets::FilePicker,
};
use winit::event::WindowEvent;
use winit::keyboard::{Key, NamedKey};

/// Crix - A skinnable UI framework
#[derive(Parser)]
#[command(name = "crix")]
#[command(about = "Run crix application bundles", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a crix application bundle
    Run {
        /// Path to the .crix bundle directory
        bundle: PathBuf,
    },
}

struct SkinApp {
    tree: UiTree,
    title: String,
    store: Store,
    dispatcher: ActionDispatcher,
    services: Services,
}

impl SkinApp {
    fn new(bundle: AppBundle) -> Result<Self, Box<dyn std::error::Error>> {
        // Load skin from bundle
        let skin = bundle.load_skin()?;
        let title = format!("{} - {}", bundle.meta.name, skin.name());

        // Build UI tree from skin
        let (tree, _window_config) = SkinBuilder::build(&skin)?;

        // Set up the store and dispatcher
        let store = Store::new();
        let mut dispatcher = ActionDispatcher::new();

        // Create Lua action handler from bundle's action scripts
        let config_adapter = bundle.to_app_config();
        println!("Loaded app: {} v{}", config_adapter.meta_name, config_adapter.meta_version);
        for action_name in config_adapter.action_names() {
            println!("  Registered action: {}", action_name);
        }

        // Build action scripts HashMap for LuaActionHandler
        let mut action_scripts = std::collections::HashMap::new();
        for action_name in bundle.action_names() {
            if let Some(path) = bundle.get_script(action_name) {
                action_scripts.insert(action_name.clone(), path.to_path_buf());
            }
        }
        let lua_handler = LuaActionHandler::from_scripts(action_scripts);
        dispatcher.add_handler(lua_handler);

        let services = Services::new();

        Ok(Self {
            tree,
            title,
            store,
            dispatcher,
            services,
        })
    }

    /// Sync text inputs to store (write dirty values).
    fn sync_inputs_to_store(&mut self) {
        let node_ids: Vec<_> = self.tree.iter_node_ids().collect();

        for id in node_ids {
            if let Some(node) = self.tree.get_mut(id) {
                if let Some(text_input) = node.widget_mut().as_any_mut().downcast_mut::<TextInput>() {
                    if text_input.is_dirty() {
                        if let Some(binding) = text_input.binding() {
                            let text = text_input.text().to_string();
                            self.store.set(binding.to_string(), text);
                        }
                        text_input.clear_dirty();
                    }
                }
            }
        }
    }

    /// Sync store values to static text widgets (update displays).
    fn sync_store_to_outputs(&mut self) {
        let node_ids: Vec<_> = self.tree.iter_node_ids().collect();

        for id in node_ids {
            if let Some(node) = self.tree.get_mut(id) {
                if let Some(static_text) = node.widget_mut().as_any_mut().downcast_mut::<StaticText>() {
                    if let Some(binding) = static_text.binding() {
                        let value = self.store.get_string(binding);
                        if !value.is_empty() && value != static_text.content() {
                            static_text.set_content(value);
                        }
                    }
                }
            }
        }
    }

    /// Dispatch an action by name.
    fn dispatch_action(&mut self, name: &str) {
        let action = Action::new(name);
        if let Err(e) = self.dispatcher.dispatch(&action, &mut self.store, &self.services) {
            eprintln!("Action error: {}", e);
        }
    }

    /// Get the action for a clicked widget (if it's a button).
    fn get_button_action(&self, node_id: crix::NodeId) -> Option<String> {
        if let Some(node) = self.tree.get(node_id) {
            // Try to get the action from a SkinButton
            if let Some(button) = node.widget().as_any().downcast_ref::<crix::skin::widgets::SkinButton>() {
                return button.action().map(|s| s.to_string());
            }
        }
        None
    }

    /// Check for FilePicker pending actions and handle them.
    fn handle_file_picker_actions(&mut self) {
        let node_ids: Vec<_> = self.tree.iter_node_ids().collect();

        for id in node_ids {
            if let Some(node) = self.tree.get_mut(id) {
                if let Some(picker) = node.widget_mut().as_any_mut().downcast_mut::<FilePicker>() {
                    if picker.has_pending_action() {
                        if let Some(action) = picker.on_select_action() {
                            if action == "launch_child_app" {
                                if let Some(path) = picker.selected_file().cloned() {
                                    picker.clear_pending_action();
                                    launch_child_app(&path);
                                }
                            } else {
                                // Handle other actions through dispatcher
                                picker.clear_pending_action();
                                // Could dispatch to Lua handler here
                            }
                        }
                        picker.clear_pending_action();
                    }
                }
            }
        }
    }
}

/// Launch a child crix app in a new process.
fn launch_child_app(path: &PathBuf) {
    println!("Launching app: {}", path.display());

    // Get the path to the current executable
    let exe = std::env::current_exe().expect("Failed to get current executable path");

    // Spawn a new process to run the child app
    match Command::new(&exe)
        .arg("run")
        .arg(path)
        .spawn()
    {
        Ok(child) => {
            println!("Launched child process with PID: {}", child.id());
        }
        Err(e) => {
            eprintln!("Failed to launch app: {}", e);
        }
    }
}

impl App for SkinApp {
    fn view(&self) -> &dyn View {
        &self.tree
    }

    fn on_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as i32;
                let y = position.y as i32;
                let hit = self.tree.hit_test(x, y);
                self.tree.set_hovered(hit);

                // Send MouseMove event to hovered widget for position tracking
                if let Some(hovered_id) = hit {
                    if let Some(node) = self.tree.get_mut(hovered_id) {
                        node.widget_mut().on_event(&WidgetEvent::MouseMove { x, y });
                    }
                }
                true
            }
            WindowEvent::MouseInput { state, .. } => {
                match state {
                    winit::event::ElementState::Pressed => {
                        // Set pressed state
                        if let Some(hovered) = self.tree.hovered() {
                            self.tree.set_pressed(Some(hovered));

                            // Focus the clicked widget (for text inputs)
                            let old_focused = self.tree.focused();
                            if old_focused != Some(hovered) {
                                // Notify old focused widget of focus loss
                                if let Some(old_id) = old_focused {
                                    if let Some(node) = self.tree.get_mut(old_id) {
                                        node.widget_mut().on_event(&WidgetEvent::FocusLost);
                                    }
                                }
                                // Set new focus
                                self.tree.set_focused(Some(hovered));
                                // Notify new widget of focus gain
                                if let Some(node) = self.tree.get_mut(hovered) {
                                    node.widget_mut().on_event(&WidgetEvent::FocusGained);
                                }
                            }
                        } else {
                            // Clicked outside any widget, clear focus
                            if let Some(old_id) = self.tree.focused() {
                                if let Some(node) = self.tree.get_mut(old_id) {
                                    node.widget_mut().on_event(&WidgetEvent::FocusLost);
                                }
                            }
                            self.tree.set_focused(None);
                        }
                    }
                    winit::event::ElementState::Released => {
                        if let Some(pressed_id) = self.tree.pressed() {
                            // Check if we're still hovering the pressed widget
                            if self.tree.hovered() == Some(pressed_id) {
                                // Get action before mutably borrowing tree
                                let action = self.get_button_action(pressed_id);

                                // Send click event to widget
                                if let Some(node) = self.tree.get_mut(pressed_id) {
                                    node.widget_mut().on_event(&WidgetEvent::Click);
                                }

                                // Handle file picker actions (must be after click event)
                                self.handle_file_picker_actions();

                                // Dispatch action if this was a button
                                if let Some(action_name) = action {
                                    // Sync inputs first
                                    self.sync_inputs_to_store();
                                    // Dispatch the action
                                    self.dispatch_action(&action_name);
                                    // Sync outputs after action
                                    self.sync_store_to_outputs();
                                }
                            }
                        }
                        self.tree.set_pressed(None);
                    }
                }
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // Convert delta to pixels (rough approximation)
                let delta_y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y * 20.0,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };

                // Route to hovered widget
                if let Some(hovered_id) = self.tree.hovered() {
                    if let Some(node) = self.tree.get_mut(hovered_id) {
                        if node.widget_mut().on_event(&WidgetEvent::MouseWheel { delta_y }) {
                            return true;
                        }
                    }
                }
                false
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if !event.state.is_pressed() {
                    return false;
                }

                // Route keyboard events to focused widget
                if let Some(focused_id) = self.tree.focused() {
                    let widget_event = match &event.logical_key {
                        Key::Named(NamedKey::Backspace) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Backspace })
                        }
                        Key::Named(NamedKey::Delete) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Delete })
                        }
                        Key::Named(NamedKey::ArrowLeft) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Left })
                        }
                        Key::Named(NamedKey::ArrowRight) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Right })
                        }
                        Key::Named(NamedKey::Home) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Home })
                        }
                        Key::Named(NamedKey::End) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::End })
                        }
                        Key::Named(NamedKey::Enter) => {
                            Some(WidgetEvent::KeyDown { key: KeyCode::Enter })
                        }
                        Key::Character(s) => {
                            // Only handle single ASCII characters
                            if s.len() == 1 {
                                let c = s.chars().next().unwrap();
                                if c as u32 >= 32 && c as u32 <= 126 {
                                    Some(WidgetEvent::CharInput { c })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        Key::Named(NamedKey::Space) => {
                            Some(WidgetEvent::CharInput { c: ' ' })
                        }
                        _ => None,
                    };

                    if let Some(widget_event) = widget_event {
                        if let Some(node) = self.tree.get_mut(focused_id) {
                            node.widget_mut().on_event(&widget_event);
                        }
                        // Sync after input
                        self.sync_inputs_to_store();
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { bundle: bundle_path } => {
            // Load the app bundle
            let bundle = match AppBundle::load(&bundle_path) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to load bundle: {}", e);
                    std::process::exit(1);
                }
            };

            // Initialize font system from bundle
            if let Err(e) = init_font(bundle.font_path(), bundle.font_size) {
                eprintln!("Failed to load font: {}", e);
                std::process::exit(1);
            }

            // Create and run the app
            let app = match SkinApp::new(bundle) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Failed to create app: {}", e);
                    std::process::exit(1);
                }
            };

            let config = RunConfig::default().with_title(&app.title);
            run(app, config);
        }
    }
}
