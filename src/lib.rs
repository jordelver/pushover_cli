pub mod cli;
mod pushover;

pub fn run(args: cli::Args) {
    pushover::send_notification(args);
}
