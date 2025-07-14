use clap::{Parser, Subcommand, arg};
use std::collections::HashMap;

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

        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,

        #[arg(short = 'd', long, help = "Enable debug output")]
        debug: bool,

        #[arg(
            long,
            help = "Set variables in key=value format (comma-separated: key1=val1,key2=val2)"
        )]
        var: Option<String>,
    },

    Validate {
        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,

        #[arg(
            long,
            help = "Set variables in key=value format (comma-separated: key1=val1,key2=val2)"
        )]
        var: Option<String>,
    },

    List {
        #[arg(short = 'v', long, help = "Enable detailed test information")]
        verbose: bool,

        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,
    },
}

impl Commands {
    /// Parse variables from the --var flag format (key1=value1,key2=value2)
    pub fn parse_variables(var_string: Option<String>) -> HashMap<String, String> {
        let mut variables = HashMap::new();

        if let Some(vars) = var_string {
            for pair in vars.split(',') {
                let pair = pair.trim();
                if let Some((key, value)) = pair.split_once('=') {
                    let key = key.trim();
                    let value = value.trim();
                    if !key.is_empty() {
                        variables.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        variables
    }
}
