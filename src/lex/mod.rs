//! Lexer Module (`nyota::lex`)
//! This module is the lexer module for `nyota`. This module contains the complete implementation of the lexer/parsing system.
//! It provides functions for tokenizing and parsing input strings into a structured format.
//! Pre-parsing is done here to ensure that the input is in a format that can be understood by relevant APIs.
//! It also provides functions for recognizing macros, heuristics, and shortcuts from user input.

pub mod mood;
pub mod sentiment;
pub mod thought;
