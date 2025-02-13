//! # Action Module (`nyota::act`)
//! This module is the action module for `nyota`. This module contains the complete implementation of the action system.
//! It provides functions for defining, loading, and executing actions.
//! The action system is used by `nyota` to provide a flexible and extensible interface for executing commands and actions.
//! It can be used to define custom actions, load actions from external sources, and execute actions based on user input.
//! The action system is designed to be modular and extensible, allowing users to define their own actions and integrate them into the program.
//!
//! ## Implementation Details
//! TBD
//!
//! ## Example
//! TBD
//!
//! ## Modules
//! TBD

use serde::{Deserialize, Serialize};

// Represents a single parameter for an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub data_type: String, // Or an enum
    pub is_required: bool,
    pub default_value: Option<String>,
    pub allowed_values: Option<Vec<String>>,
}

// Represents a complete action schema
#[derive(Debug, Clone)] //Serialize, Deserialize
pub struct ActionSchema {
    pub name: String,
    pub description: String,
    pub similes: Vec<String>,
    pub examples: Vec<String>,
    pub parameters: Vec<Parameter>,
    pub is_public: bool,
    // #[serde(skip_serializing_if = "Option::is_none")]
    //  pub callback: Option<fn(Vec<String>) -> String>, // Function pointer
    pub response_format: String, // Or an enum
}

// Example usage (for a built-in action)
pub fn create_get_balance_action() -> ActionSchema {
    ActionSchema {
        name: "get_balance".to_string(),
        description: "Retrieves the balance of a given wallet address on a specified blockchain."
            .to_string(),
        similes: vec![
            "check balance".to_string(),
            "what's my balance".to_string(),
            "show funds".to_string(),
        ],
        examples: vec![
            "get my solana balance".to_string(),
            "check eth balance on mainnet".to_string(),
            "show my balance".to_string(),
        ],
        parameters: vec![
            Parameter {
                name: "chain".to_string(),
                description: "The blockchain to query (e.g., 'solana', 'ethereum').".to_string(),
                data_type: "string".to_string(),
                is_required: true,
                default_value: None,
                allowed_values: Some(vec![
                    "solana".to_string(),
                    "ethereum".to_string(),
                    "bitcoin".to_string(),
                ]),
            },
            Parameter {
                name: "address".to_string(),
                description: "The wallet address to check.".to_string(),
                data_type: "address".to_string(), // You might want a custom type for this
                is_required: true,
                default_value: None,
                allowed_values: None,
            },
        ],
        is_public: true,
        //   callback: Some(get_balance_callback), // The actual function
        response_format: "json".to_string(),
    }
}

// Dummy callback function for demonstration
fn get_balance_callback(params: Vec<String>) -> String {
    // In a real implementation, this function would:
    // 1. Parse the parameters (chain, address).
    // 2. Call the appropriate blockchain API.
    // 3. Format the result as JSON.
    println!("get_balance_callback called with params: {:?}", params); // Debug print
    format!(
        "{{ \"chain\": \"{}\", \"address\": \"{}\", \"balance\": 123.45 }}",
        params[0], params[1]
    )
}

// Example of loading from JSON (for user-defined actions)
//pub fn load_action_schema_from_json(json_str: &str) -> Result<ActionSchema, serde_json::Error> {
//    serde_json::from_str(json_str)
//}
