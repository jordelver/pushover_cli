use reqwest::blocking::Client;
use serde::Serialize;

use crate::cli::Args;

static PUSHOVER_API_ENDPOINT: &str = "https://api.pushover.net/1/messages.json";

#[derive(Debug, Serialize)]
struct Payload {
    token: String,
    user: String,
    message: String,
}

pub type PushoverResult = Result<(), Box<dyn std::error::Error>>;

pub fn send_notification(args: Args) -> PushoverResult {
    let payload = Payload {
        token: args.token,
        user: args.user,
        message: args.message,
    };

    Client::new()
        .post(PUSHOVER_API_ENDPOINT)
        .form(&payload)
        .send()?
        .error_for_status()?;

    Ok(())
}
