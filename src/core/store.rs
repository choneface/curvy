use std::collections::HashMap;

/// A dynamic value that can be stored in the Store.
/// Designed to be language-agnostic for future scripting support.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    /// Create a Value from a string.
    pub fn string(s: impl Into<String>) -> Self {
        Value::String(s.into())
    }

    /// Create a Value from a number.
    pub fn number(n: f64) -> Self {
        Value::Number(n)
    }

    /// Create a Value from a bool.
    pub fn bool(b: bool) -> Self {
        Value::Bool(b)
    }

    /// Try to get as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as a number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get as a bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if the value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Convert to string representation.
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Null => String::new(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => {
                // Format nicely: no trailing zeros for integers
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
        }
    }

    /// Try to parse as a number (for string values).
    pub fn try_parse_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(n as f64)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

/// Centralized key-value store for application state.
/// Widgets read/write named keys; actions process and update state.
#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, Value>,
}

impl Store {
    /// Create a new empty store.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Get a value by key.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Set a value by key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.data.insert(key.into(), value.into());
    }

    /// Remove a key from the store.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.data.remove(key)
    }

    /// Check if a key exists.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Get a string value, returning empty string if not found or wrong type.
    pub fn get_str(&self, key: &str) -> &str {
        self.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
    }

    /// Get a string value, converting other types to string.
    pub fn get_string(&self, key: &str) -> String {
        self.get(key)
            .map(|v| v.to_string_value())
            .unwrap_or_default()
    }

    /// Get a number value, returning None if not found or can't parse.
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.get(key).and_then(|v| v.try_parse_number())
    }

    /// Get a bool value, returning false if not found or wrong type.
    pub fn get_bool(&self, key: &str) -> bool {
        self.get(key)
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Iterate over all keys.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    /// Clear all data.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_basic() {
        let mut store = Store::new();
        store.set("name", "Alice");
        store.set("age", 30.0);
        store.set("active", true);

        assert_eq!(store.get_str("name"), "Alice");
        assert_eq!(store.get_number("age"), Some(30.0));
        assert!(store.get_bool("active"));
    }

    #[test]
    fn test_value_conversions() {
        let v = Value::string("42");
        assert_eq!(v.try_parse_number(), Some(42.0));

        let v = Value::number(3.14);
        assert_eq!(v.to_string_value(), "3.14");

        let v = Value::number(42.0);
        assert_eq!(v.to_string_value(), "42");
    }
}
