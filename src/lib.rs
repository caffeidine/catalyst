pub mod dsl;

use clap::{Parser, Subcommand};

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
        Commands::Run { filter, verbose } => {
            println!("Running API tests...");
            let mut runner = dsl::runner::TestRunner::new();
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
