use std::collections::HashMap;

use super::store::{Store, Value};

/// An action that triggers app logic.
/// Actions are the main hook point for future scripting integration.
#[derive(Debug, Clone)]
pub struct Action {
    /// The action name (e.g., "calculate_blend").
    pub name: String,
    /// Optional payload with additional data.
    pub payload: HashMap<String, Value>,
}

impl Action {
    /// Create a new action with just a name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            payload: HashMap::new(),
        }
    }

    /// Create an action with a payload.
    pub fn with_payload(name: impl Into<String>, payload: HashMap<String, Value>) -> Self {
        Self {
            name: name.into(),
            payload,
        }
    }

    /// Add a value to the payload.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.payload.insert(key.into(), value.into());
        self
    }

    /// Get a value from the payload.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.payload.get(key)
    }

    /// Get a string from the payload.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.payload.get(key).and_then(|v| v.as_str())
    }

    /// Get a number from the payload.
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.payload.get(key).and_then(|v| v.as_number())
    }
}

/// Error type for action handling.
#[derive(Debug)]
pub enum ActionError {
    /// Action was not handled by any handler.
    NotHandled(String),
    /// An error occurred during action processing.
    Failed(String),
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::NotHandled(name) => write!(f, "Action not handled: {}", name),
            ActionError::Failed(msg) => write!(f, "Action failed: {}", msg),
        }
    }
}

impl std::error::Error for ActionError {}

/// Services available to action handlers.
/// Reserved for future expansion (time, random, network, etc.).
#[derive(Debug, Default)]
pub struct Services {
    // Currently empty - placeholder for future services
}

impl Services {
    /// Create a new services instance.
    pub fn new() -> Self {
        Self {}
    }
}

/// Trait for handling actions.
/// Implement this to create custom action handlers.
///
/// The design allows swapping implementations without changing widgets:
/// - Rust handlers for native logic
/// - Future: Lua/JS script handlers
pub trait ActionHandler {
    /// Handle an action.
    /// Returns Ok(true) if the action was handled, Ok(false) if not.
    fn handle(
        &mut self,
        action: &Action,
        store: &mut Store,
        services: &Services,
    ) -> Result<bool, ActionError>;
}

/// A composite action handler that chains multiple handlers.
pub struct ActionDispatcher {
    handlers: Vec<Box<dyn ActionHandler>>,
}

impl ActionDispatcher {
    /// Create a new empty dispatcher.
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    /// Add a handler to the chain.
    pub fn add_handler(&mut self, handler: impl ActionHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    /// Dispatch an action to all handlers until one handles it.
    pub fn dispatch(
        &mut self,
        action: &Action,
        store: &mut Store,
        services: &Services,
    ) -> Result<bool, ActionError> {
        for handler in &mut self.handlers {
            if handler.handle(action, store, services)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl Default for ActionDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
