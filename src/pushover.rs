pub mod error;
mod response_body;

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;

use crate::cli::Args;
use error::PushoverError;
use error::PushoverError::ApiError;
use response_body::ResponseBody;

static PUSHOVER_API_ENDPOINT: &str = "https://api.pushover.net/1/messages.json";

#[derive(Debug, Serialize)]
struct Payload {
    token: String,
    user: String,
    message: String,
}

impl Payload {
    pub fn new(args: Args) -> Self {
        Self {
            token: args.token,
            user: args.user,
            message: args.message,
        }
    }
}

pub fn send_notification(args: Args) -> Result<(), PushoverError> {
    let payload = Payload::new(args);

    let response = Client::new()
        .post(PUSHOVER_API_ENDPOINT)
        .form(&payload)
        .send()?;

    let status = response.status();
    let body: ResponseBody = response.json()?;

    match status {
        StatusCode::OK => Ok(()),
        _ => Err(ApiError(body)),
    }
}
