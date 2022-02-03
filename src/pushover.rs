use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;

use crate::cli::Args;

static PUSHOVER_API_ENDPOINT: &str = "https://api.pushover.net/1/messages.json";

#[derive(Debug, Serialize)]
struct Payload {
    token: String,
    user: String,
    message: String,
}

pub fn send_notification(args: Args) {
    let payload = Payload {
        token: args.token,
        user: args.user,
        message: args.message,
    };

    let response = Client::new()
        .post(PUSHOVER_API_ENDPOINT)
        .form(&payload)
        .send()
        .unwrap();

    match response.status() {
        StatusCode::OK => println!("Notification sent"),
        _ => println!("Failed to send notification"),
    }
}
