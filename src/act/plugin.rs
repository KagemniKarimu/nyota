use crate::act::schema::ActionSchema;
use anyhow::Error;
use std::collections::HashMap;

pub trait Plugin {
    fn name(&self) -> &str;
    fn category(&self) -> PluginCategory;
    fn action_schema(&self) -> &ActionSchema;
    fn execute(&self, params: Option<&HashMap<String, String>>) -> Result<(), Error>;
    fn validate(&self) -> Result<(), Error>;
    fn dependencies(&self) -> Vec<String>;
    fn cleanup(&self) -> Result<(), Error>;
}

pub enum PluginCategory {
    // Core functionality
    System,      // System operations
    Wallet,      // Wallet management
    Transaction, // Transaction handling

    // Extensions
    ApiIntegration, // External API interactions
    UserCommand,    // Custom user commands
    Utility,        // Helper functions

    // Advanced
    Compound, // Multiple action composition
    Custom,   // User-defined category
}
