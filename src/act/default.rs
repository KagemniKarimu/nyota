use crate::act::schema::{ActionSchema, ActionType};
use std::collections::HashMap;

use crate::act::plugin::{Plugin, PluginCategory};
use anyhow::Error;

struct DefaultPlugins {
    model_switch: ModelSwitchPlugin,
    // TODO: provider_change: ProviderChangePlugin,
}

impl DefaultPlugins {
    pub fn new() -> Self {
        Self {
            model_switch: ModelSwitchPlugin::new(),
            // TODO: provider_change: ProviderChangePlugin::new(),
        }
    }
}

pub struct ModelSwitchPlugin {
    schema: ActionSchema,
    // Could hold reference to API adapter if needed
}

impl ModelSwitchPlugin {
    pub fn new() -> Self {
        // Create schema for model switching
        let mut params = HashMap::new();
        params.insert(
            String::from("model"),
            String::from("Name of the model to switch to"),
        );

        let schema = ActionSchema::new(
            1,
            String::from("model_switch"),
            String::from("Switch AI Models"),
            String::from("Switch the current AI model to a different supported model"),
            ActionType::ApiInteraction,
            params,
        );

        Self { schema }
    }
}

impl Plugin for ModelSwitchPlugin {
    fn name(&self) -> &str {
        &self.schema.name
    }

    fn category(&self) -> PluginCategory {
        PluginCategory::System
    }

    fn execute(&self, params: Option<&HashMap<String, String>>) -> Result<(), Error> {
        let params = params.ok_or_else(|| Error::msg("Parameters required for model switch"))?;

        // Get model name from params
        let new_model = params
            .get("model")
            .ok_or_else(|| Error::msg("Model name not provided"))?;

        // Validate model exists in supported models
        if !crate::api::constants::SUPPORTED_MODELS.contains_key(new_model.as_str())
            && !new_model.starts_with("openrouter/")
            && !new_model.starts_with("ollama/")
        {
            return Err(Error::msg(format!("Unsupported model: {}", new_model)));
        }

        // Here you'd actually switch the model
        // This might need to be integrated with your API adapter

        println!("Switched to model: {}", new_model);
        Ok(())
    }

    fn validate(&self) -> Result<(), Error> {
        // Validate plugin is properly configured
        // Check if schema is valid
        // Verify required dependencies
        Ok(())
    }

    fn dependencies(&self) -> Vec<String> {
        // List any other plugins this depends on
        Vec::new()
    }

    fn cleanup(&self) -> Result<(), Error> {
        // Any cleanup needed when plugin is unloaded
        Ok(())
    }

    fn action_schema(&self) -> &ActionSchema {
        &self.schema
    }
}
