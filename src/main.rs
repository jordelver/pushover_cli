use clap::Parser;
use pushover_cli::Args;

fn main() {
    let args = Args::parse();

    pushover_cli::send_notification(args)
}
