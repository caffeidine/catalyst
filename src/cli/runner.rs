use super::commands::{Commands, Opts};
use crate::checker::{list_tests, validate};
use crate::core::runner::TestRunner;
use crate::utils::debug;

/// Run the CLI application
/// 
/// # Panics
/// Will panic if the tokio runtime cannot be created
pub fn run(opts: Opts) {
    match opts.command {
        Commands::Run {
            filter,
            disable_color,
            verbose,
            file,
            debug: debug_enabled,
            var,
            no_fail_summary,
        } => {
            if debug_enabled {
                debug::enable_debug();
            }
            println!("Running API tests...");
            tokio::runtime::Runtime::new().unwrap().block_on(run_tests(
                filter,
                verbose,
                disable_color,
                file,
                var,
                no_fail_summary,
            ));
        }
        Commands::Validate { file, var } => {
            println!("Validating tests configuration...");
            run_validate(file, var);
        }
        Commands::List { verbose, file } => {
            println!("Listing available tests...");
            list_tests(verbose, file.as_deref());
        }
    }
}

pub fn run_validate(file: Option<String>, _var: Option<String>) {
    // Note: Variables are not used in validation, only for actual test execution
    match file {
        Some(f) => validate(Some(&f)),
        None => validate(None),
    }
}

pub async fn run_tests(
    filter: Option<String>,
    verbose: bool,
    disable_color: bool,
    file: Option<String>,
    var: Option<String>,
    no_fail_summary: bool,
) {
    let mut runner = TestRunner::new(disable_color);
    runner.execute_tests(filter, verbose, file, var, no_fail_summary).await;
}
