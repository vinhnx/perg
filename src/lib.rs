//! # perg
//!
//! A fast text search tool similar to grep, written in Rust.
//!
//! This crate provides the core functionality for searching text patterns
//! in files using regular expressions.

pub mod cli;
pub mod error;
pub mod search;

// Declare external dependencies for use in modules
extern crate clap;
extern crate regex;
extern crate walkdir;

// Re-export commonly used types
pub use cli::Args;
pub use error::{PergError, Result};
pub use search::{search_file, search_paths, SearchConfig};
