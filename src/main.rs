use clap::Parser;
use pushover_cli::cli;

fn main() {
    let args = cli::Args::parse();

    pushover_cli::run(args)
}
