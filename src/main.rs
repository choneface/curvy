use std::path::Path;

use curvy::{
    run, init_font, Action, ActionDispatcher, ActionError, ActionHandler, App, KeyCode,
    LoadedSkin, RunConfig, Services, SkinBuilder, StaticText, Store, TextInput, UiTree,
    View, WidgetEvent,
};
use winit::event::WindowEvent;
use winit::keyboard::{Key, NamedKey};

/// Action handler for the blend calculator demo.
/// Calculates how much E85 to add to reach a target ethanol percentage.
struct BlendCalculatorHandler;

impl ActionHandler for BlendCalculatorHandler {
    fn handle(
        &mut self,
        action: &Action,
        store: &mut Store,
        _services: &Services,
    ) -> Result<bool, ActionError> {
        match action.name.as_str() {
            "calculate_blend" => {
                // Read inputs from store
                let current_pct = store.get_number("inputs.current_ethanol_pct").unwrap_or(0.0);
                let target_pct = store.get_number("inputs.target_ethanol_pct").unwrap_or(0.0);
                let current_fuel = store.get_number("inputs.current_fuel_liters").unwrap_or(0.0);

                // E85 is approximately 85% ethanol
                const E85_ETHANOL_PCT: f64 = 85.0;

                // Calculate E85 needed: solve for x where:
                // (current_pct * current_fuel + E85_ETHANOL_PCT * x) / (current_fuel + x) = target_pct
                // Rearranging: x = (target_pct - current_pct) * current_fuel / (E85_ETHANOL_PCT - target_pct)
                let result = if target_pct >= E85_ETHANOL_PCT {
                    // Can't exceed E85 percentage
                    f64::INFINITY
                } else if target_pct <= current_pct {
                    // Already at or above target
                    0.0
                } else if current_fuel <= 0.0 {
                    0.0
                } else {
                    (target_pct - current_pct) * current_fuel / (E85_ETHANOL_PCT - target_pct)
                };

                // Format the result
                let result_str = if result.is_infinite() {
                    "N/A".to_string()
                } else {
                    format!("{:.2}", result)
                };

                store.set("outputs.e85_to_add_liters", result_str);
                println!("Calculated: {} liters of E85 needed", store.get_string("outputs.e85_to_add_liters"));

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

struct SkinApp {
    tree: UiTree,
    title: String,
    store: Store,
    dispatcher: ActionDispatcher,
    services: Services,
}

impl SkinApp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load skin from directory
        let skin = LoadedSkin::load(Path::new("skins/classic/skin.json"))?;
        let title = skin.name().to_string();

        // Build UI tree from skin
        let (tree, _window_config) = SkinBuilder::build(&skin)?;

        // Set up the store and dispatcher
        let store = Store::new();
        let mut dispatcher = ActionDispatcher::new();
        dispatcher.add_handler(BlendCalculatorHandler);
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
    fn get_button_action(&self, node_id: curvy::NodeId) -> Option<String> {
        if let Some(node) = self.tree.get(node_id) {
            // Try to get the action from a SkinButton
            if let Some(button) = node.widget().as_any().downcast_ref::<curvy::skin::widgets::SkinButton>() {
                return button.action().map(|s| s.to_string());
            }
        }
        None
    }
}

impl App for SkinApp {
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
    // Initialize font system - requires a TTF file
    init_font(Path::new("fonts/font.ttf"), 16.0)
        .expect("Failed to load font. Please place a TTF file at fonts/font.ttf");

    let app = SkinApp::new().expect("Failed to load skin");
    let config = RunConfig::default().with_title(&app.title);

    run(app, config);
}
