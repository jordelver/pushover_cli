use clap::Parser;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;

/// Send notifications using Pushover
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Pushover user
    #[clap(short, long)]
    user: String,

    /// Pushover token
    #[clap(short, long)]
    token: String,
}

static PUSHOVER_API_ENDPOINT: &str = "https://api.pushover.net/1/messages.json";

#[derive(Debug, Serialize)]
struct Payload {
    token: String,
    user: String,
    message: String,
}

fn main() {
    let args = Args::parse();

    let payload = Payload {
        token: args.token.to_string(),
        user: args.user.to_string(),
        message: "hello, world".to_string(),
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
