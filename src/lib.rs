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

pub mod dsl;

use clap::{Parser, Subcommand, arg};

#[derive(Parser)]
#[command(name = "catalyst")]
#[command(version, about = "A lightweight API testing tool", long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(short = 'f', long, help = "Filter by test name")]
        filter: Option<String>,

        #[arg(long, default_value = "false", help = "Disable colored output")]
        disable_color: bool,

        #[arg(short = 'v', long, help = "Enable verbose output")]
        verbose: bool,
    },

    Validate,

    List {
        #[arg(short = 'v', long, help = "Enable detailed test information")]
        verbose: bool,
    },
}

/// Execute the selected command
pub fn run(opts: Opts) {
    match opts.command {
        Commands::Run {
            filter,
            disable_color,
            verbose,
        } => {
            println!("Running API tests...");
            let mut runner = dsl::runner::TestRunner::new(disable_color);
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(runner.execute_tests(filter, verbose));
        }
        Commands::Validate => {
            println!("Validating tests configuration...");
            dsl::validator::validate();
        }
        Commands::List { verbose } => {
            println!("Listing available tests...");
            dsl::parser::list_tests(verbose);
        }
    }
}
