//! # Catalyst
//!
//! **Catalyst** is a lightweight and extensible API testing tool. It allows you to define and execute HTTP API tests through a declarative configuration file.
//!
//! ## Features
//!
//! - **Test Definition**: Configure API test scenarios using a configuration file.
//! - **Variable Management**: Chain tests by extracting and storing variables (e.g., cookies, JSON data).
//! - **Configuration Validation**: Pre-run syntax and value checks for test configurations.
//! - **Advanced Assertions**: Flexible response validation with multiple assertion types.
//!
//! ## Architecture
//!
//! The codebase is organized into logical modules:
//! - **parser**: Test file parsing and validation
//! - **engine**: Core test execution, validation, and assertions
//! - **http**: HTTP client and request handling
//! - **models**: Data structures and types
//! - **core**: High-level orchestration
//! - **cli**: Command-line interface
//! - **utils**: Utility functions

pub mod checker;
pub mod cli;
pub mod core;
pub mod engine;
pub mod error;
pub mod http;
pub mod models;
pub mod utils;

// Re-export commonly used items
pub use checker::{list_tests, parse_tests, validate};
pub use cli::{Commands, Opts};
pub use core::runner::TestRunner;
