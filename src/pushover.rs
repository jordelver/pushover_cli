pub mod error;
mod payload;
mod response_body;

use reqwest::blocking::Client;
use reqwest::StatusCode;

use error::PushoverError;
use response_body::ResponseBody;

static PUSHOVER_API_ENDPOINT: &str = "https://api.pushover.net/1/messages.json";

pub fn send_notification(args: super::cli::Args) -> Result<(), PushoverError> {
    let payload = payload::Payload::new(args);

    let response = Client::new()
        .post(PUSHOVER_API_ENDPOINT)
        .form(&payload)
        .send()?;

    let status = response.status();
    let body: ResponseBody = response.json()?;

    match status {
        StatusCode::OK => Ok(()),
        _ => Err(PushoverError::ApiError(body)),
    }
}
