use catalyst::cli::{Opts, run};
use clap::Parser;

fn main() {
    run(Opts::parse());
}
