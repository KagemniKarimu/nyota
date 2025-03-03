//! # Context Module (`nyota::ctx`)
//!
//! This module is the context module for `nyota`. This module contains the complete implementation of the context system.
//! It provides functions for managing the context of the program, including the user's state, preferences, and history.
//! The context system is used by `nyota` to provide a persistent and flexible interface for storing and retrieving user data.

pub mod convo;
pub mod hybrid_store;
pub mod mem_store;
pub mod redis_store;

#[cfg(test)]
mod tests {
    #[test]
    fn test_cache_module_imports() {
        // Check that all submodules are properly exported.
        use crate::cache::convo;
        use crate::cache::hybrid_store;
        use crate::cache::mem_store;
        use crate::cache::redis_store;
    }
}
