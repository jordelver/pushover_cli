pub mod cli;
mod pushover;

pub fn run(args: cli::Args) {
    match pushover::send_notification(args) {
        Ok(()) => println!("Notification sent"),
        Err(_) => println!("Error"),
    }
}
