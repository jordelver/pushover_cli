pub mod cli;
mod pushover;

use pushover::error::PushoverError;

pub fn run(args: cli::Args) {
    match pushover::send_notification(args) {
        Ok(()) => {
            println!("Notification sent")
        }
        Err(err) => match err {
            PushoverError::ApiError => println!("API Error: {}", err),
            PushoverError::HttpError => println!("HTTP Error: {}", err),
        },
    }
}
