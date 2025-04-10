//! # Catalyst
//!
//! **Catalyst** is a lightweight and extensible API testing tool. It allows you to define and execute HTTP API tests through a declarative configuration file.
//!
//! ## Features
//!
//! - **Test Definition**: Configure API test scenarios using a configuration file.
//! - **Variable Management**: Chain tests by extracting and storing variables (e.g., cookies, JSON data).
//! - **Configuration Validation**: Pre-run syntax and value checks for test configurations.
//!
//! ## Installation
//!
//! ```sh
//! cargo install catalyst
//! ```
//!
//! ## Usage
//!
//! Create your test file in your project `.catalyst/tests.toml`
//!
//! ### Example test configuration
//!
//! ```toml
//! [config]
//! base_url = "http://localhost:8080"
//! default_headers = { "User-Agent" = "Catalyst", "Content-Type" = "application/json" }
//!
//! [[tests]]
//! name = "Example Test"
//! method = "GET"
//! endpoint = "/api/example"
//! expected_status = 200
//! ```
//!
//! ### Running Tests
//!
//! Execute the tests from the command line:
//!
//! ```sh
//! catalyst run
//! ```
//!
//! ### To list all tests or validate your configuration, use:
//!
//! ```sh
//! catalyst list --verbose
//! catalyst validate
//! ```

pub mod cli;
pub mod core;
pub mod http;
pub mod models;
pub mod parser;
pub mod utils;

// Re-export CLI components for backward compatibility
pub use cli::{Commands, Opts, run};
