use super::commands::{Commands, Opts};
use crate::core;
use crate::parser;

pub fn run(opts: Opts) {
    match opts.command {
        Commands::Run {
            filter,
            disable_color,
            verbose,
            file,
        } => {
            println!("Running API tests...");
            let mut runner = core::runner::TestRunner::new(disable_color);
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(runner.execute_tests(filter, verbose, file));
        }
        Commands::Validate { file } => {
            println!("Validating tests configuration...");
            core::validator::validate(file.as_deref());
        }
        Commands::List { verbose, file } => {
            println!("Listing available tests...");
            parser::list_tests(verbose, file.as_deref());
        }
    }
}
