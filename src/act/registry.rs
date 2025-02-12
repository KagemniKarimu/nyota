use anyhow::Error;

use crate::act::schema::ActionSchema;
use std::collections::HashMap;

struct PluginRegistry {
    plugins: HashMap<String, ActionSchema>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_action(&mut self, action: ActionSchema) -> Result<(), Error> {
        match self.plugins.insert(String::from(&action.name), action) {
            Some(_) => Ok(()),
            None => Err(Error::msg("[REGISTRY] Unable to register action")),
        }
    }

    pub fn deregister_action(&mut self, name: &str) -> Result<(), Error> {
        match self.plugins.remove(name) {
            Some(_) => Ok(()),
            None => Err(Error::msg("[REGISTRY] Unable to deregister action")),
        }
    }

    pub fn lookup_plugin(&self, name: &str) -> Option<&ActionSchema> {
        self.plugins.get(name)
    }

    pub fn plugin_exists(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    pub fn list_action_schemas(&self) -> Vec<&ActionSchema> {
        self.plugins.values().collect()
    }

    pub fn list_plugin_names(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    pub fn load_plugins(&mut self, plugins: Vec<ActionSchema>) -> Result<(), Error> {
        for plugin in plugins {
            self.register_action(plugin)?;
        }
        Ok(())
    }

    pub fn unload_plugins(&mut self, plugins: Vec<String>) -> Result<(), Error> {
        for plugin in plugins {
            self.deregister_action(&plugin)?;
        }
        Ok(())
    }
}
