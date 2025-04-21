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

        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,

        #[arg(short = 'd', long, help = "Enable debug output")]
        debug: bool,
    },

    Validate {
        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,
    },

    List {
        #[arg(short = 'v', long, help = "Enable detailed test information")]
        verbose: bool,

        #[arg(long, help = "Specify a custom test file path")]
        file: Option<String>,
    },
}
