//! # perg
//!
//! A fast, feature-rich text search tool similar to grep, written in Rust.
//!
//! This crate provides the core functionality for searching text patterns
//! in files using regular expressions with support for:
//! 
//! - Basic pattern matching with regular expressions
//! - Case-insensitive matching (`-i` flag)
//! - Line number display (`-n` flag)
//! - Recursive directory searching (`-r` flag)
//! - Context lines around matches (`-B`, `-A`, `-C` flags)
//! - Count matching lines (`-c` flag)
//! - Show only matching parts (`-o` flag)
//! - Limit number of matches (`-m` flag)
//! - Extended regular expressions (`-E` flag)
//! - Colorized output (`--color` flag)
//! - Invert match (`-v` flag)
//! - Files with/without matches listing (`-l`/`-L` flags)
//! - Reading from stdin when no file paths are provided

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
pub use search::{search_file, search_paths, search_stdin, SearchConfig};
