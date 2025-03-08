# Simplifying Your Action System

Your current action system has good ideas but has become fragmented with multiple approaches: plugins, schemas, macros, and registry. Let's consolidate this into a more cohesive design.

## Core Principles for Redesign

1. **Single, unified abstraction** for actions
2. **Trait-based approach** for extensibility
3. **Minimal type duplication**
4. **Built-in registry** that's easy to use
5. **Macros for boilerplate reduction**

## Proposed Solution

Here's a cleaner approach:

```rust
// nyota/src/act/mod.rs
pub mod actions; // Built-in actions
mod macros;     // Procedural macros for defining actions (separate crate if needed)

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::{Error, Result};
use lazy_static::lazy_static;

/// Parameter for an action
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<String>,
}

/// Types of actions available
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    Query,        // Read-only operations
    Transaction,  // Blockchain transactions
    System,       // System operations
    ApiCall,      // External API calls
    Composite,    // Multiple actions combined
}

/// Core trait for all actions
pub trait Action: Send + Sync {
    /// Unique identifier for this action
    fn name(&self) -> &str;

    /// Type of action
    fn action_type(&self) -> ActionType;

    /// Human-readable description
    fn description(&self) -> &str;

    /// Parameters required for this action
    fn parameters(&self) -> Vec<Parameter>;

    /// Execute the action with given parameters
    fn execute(&self, params: &HashMap<String, String>) -> Result<String>;

    /// Examples of using this action
    fn examples(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Alternative phrases that might trigger this action
    fn aliases(&self) -> Vec<&str> {
        Vec::new()
    }
}

/// Registry for all available actions
pub struct ActionRegistry {
    actions: HashMap<String, Arc<dyn Action>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn register<A: Action + 'static>(&mut self, action: A) -> Result<()> {
        let name = action.name().to_string();
        if self.actions.contains_key(&name) {
            return Err(Error::msg(format!("Action '{}' already registered", name)));
        }
        self.actions.insert(name, Arc::new(action));
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Action>> {
        self.actions.get(name).cloned()
    }

    pub fn list(&self) -> Vec<&str> {
        self.actions.keys().map(|k| k.as_str()).collect()
    }

    pub fn execute(&self, name: &str, params: &HashMap<String, String>) -> Result<String> {
        match self.get(name) {
            Some(action) => action.execute(params),
            None => Err(Error::msg(format!("Action '{}' not found", name)))
        }
    }
}

// Global registry
lazy_static! {
    static ref REGISTRY: RwLock<ActionRegistry> = RwLock::new(ActionRegistry::new());
}

/// Register a new action
pub fn register<A: Action + 'static>(action: A) -> Result<()> {
    REGISTRY.write().unwrap().register(action)
}

/// Execute an action by name
pub fn execute(name: &str, params: &HashMap<String, String>) -> Result<String> {
    REGISTRY.read().unwrap().execute(name, params)
}

/// List all registered actions
pub fn list_actions() -> Vec<String> {
    REGISTRY.read().unwrap().list().into_iter().map(String::from).collect()
}

// Re-export common types
pub use self::actions::*;
```

Now for a simple procedural macro to make action definition clean:

```rust
// nyota/src/act/macros.rs (or a separate crate)

/// Example usage for a proc macro (conceptual):
///
/// #[action(
///    name = "model_switch",
///    description = "Switch to a different AI model",
///    action_type = "System"
/// )]
/// fn switch_model(params: &HashMap<String, String>) -> Result<String> {
///     // Implementation
///     Ok(format!("Switched to model: {}", params.get("model").unwrap_or(&"default".to_string())))
/// }
```

Example implementation of a built-in action:

```rust
// nyota/src/act/actions/model_switch.rs
use crate::act::{Action, ActionType, Parameter};
use anyhow::{Error, Result};
use std::collections::HashMap;

pub struct ModelSwitchAction;

impl Action for ModelSwitchAction {
    fn name(&self) -> &str {
        "model_switch"
    }

    fn action_type(&self) -> ActionType {
        ActionType::System
    }

    fn description(&self) -> &str {
        "Switch the current AI model to a different supported model"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "model".to_string(),
                description: "Name of the model to switch to".to_string(),
                required: true,
                default_value: None,
            }
        ]
    }

    fn execute(&self, params: &HashMap<String, String>) -> Result<String> {
        let new_model = params.get("model")
            .ok_or_else(|| Error::msg("Model name not provided"))?;

        // Validation logic
        if !crate::api::constants::SUPPORTED_MODELS.contains_key(new_model.as_str())
            && !new_model.starts_with("openrouter/")
            && !new_model.starts_with("ollama/")
        {
            return Err(Error::msg(format!("Unsupported model: {}", new_model)));
        }

        // Actual model switching logic would go here

        Ok(format!("Switched to model: {}", new_model))
    }

    fn examples(&self) -> Vec<&str> {
        vec![
            "switch to gpt-4",
            "change model to claude-3",
            "use llama-3 model"
        ]
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["change_model", "use_model", "switch_model"]
    }
}
```

## Main advantages:

1. **Single trait** (`Action`) unifies all the behavior
2. **Less redundancy** - no need for separate schema and implementation
3. **Global registry** with simple API
4. **Type safety** with trait objects
5. **Extensibility** - easy to add new actions
6. **Modularity** - actions are defined in separate files
7. **Thread-safety** with `RwLock` and `Arc`

This approach gives you:
- Clear interfaces for extending the system
- A unified way to think about actions
- Simpler code organization
- Reduced duplication
- Still flexible enough for complex behavior
