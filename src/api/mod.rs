//! # API Module (`nyota::api`)
//! This module is the API module for `nyota`. This module contains the complete implementation of the API system.
//! It provides functions for interacting with various API providers and serializing, deserializing, and submitting requests.
//! The API system is used by `nyota` to provide a flexible and extensible interface for executing commands and actions.
//!
//! ## Implementation Details
//! The API system is implemented using the `reqwest` crate for making HTTP requests and the `serde` crate for serializing and deserializing JSON data.
//! Each request is formulated into a JSON object with values for the method, URL, headers, and body.
//! The response is deserialized into a JSON object and returned to the caller.
//! This is handled by an `Adapter` which gives public interfaces for other modules to perform operations.
//! Though ostensibly it resembles the singleton pattern when a default adapter is instanciated. The idea was to have multiple adapters at use per program instance.
//!
//! ## Example
//! ```rust
//! use nyota::api::Adapter;
//!
//! // Initialize an adapter
//! let adapter = Adapter::new();
//!
//! // Send Message to the Currently-in focus LLM
//! let response = adapter.send_to_llm(String::from(user_msg)).await;

pub mod constants;
pub mod utilities;
