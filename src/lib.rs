pub mod cli;
mod pushover;

use pushover::error::PushoverError;

pub fn run(args: cli::Args) {
    match pushover::send_notification(args) {
        Ok(()) => {
            println!("Notification sent")
        }
        Err(err) => match err {
            PushoverError::ApiError(body) => eprintln!("API Error\n{:#?}", body),
            PushoverError::HttpError(original) => eprintln!("HTTP Error\n{:?}", original),
        },
    }
}
