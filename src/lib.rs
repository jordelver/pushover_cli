use clap::Parser;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;

/// Send notifications using Pushover
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Pushover user
    #[clap(short, long)]
    user: String,

    /// Pushover token
    #[clap(short, long)]
    token: String,

    /// Message to send
    #[clap(short, long)]
    message: String,
}

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
