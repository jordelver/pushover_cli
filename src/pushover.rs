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
    send_request(PUSHOVER_API_ENDPOINT, payload)
}

pub fn send_request(url: &str, payload: payload::Payload) -> Result<(), PushoverError> {
    let response = Client::new().post(url).form(&payload).send()?;
    let status = response.status();
    let body: ResponseBody = response.json()?;

    match status {
        StatusCode::OK => Ok(()),
        _ => Err(PushoverError::ApiError(body)),
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    mod stubs {
        pub fn missing_message() -> Vec<u8> {
            r#"{
                "message": "cannot be blank",
                "status": 0,
                "request": "ad82d72c-f9f1-468e-98cb-4a1b901f365b",
                "errors": [
                    "message cannot be blank"
                ]
            }"#
            .as_bytes()
            .to_owned()
        }

        pub fn invalid_token() -> Vec<u8> {
            r#"{
                "message": "",
                "status": 0,
                "request": "65d05da9-b475-40f5-bb55-386907082a31",
                "errors": [
                    "application token is invalid"
                ]
            }"#
            .as_bytes()
            .to_owned()
        }

        pub fn invalid_user() -> Vec<u8> {
            r#"{
                "message": "",
                "status": 0,
                "request": "69fd9ac0-dda0-4527-b292-b1226d18b9f0",
                "errors": [
                    "user identifier is not a valid user, group, or subscribed user key"
                ]
            }"#
            .as_bytes()
            .to_owned()
        }
    }

    fn payload() -> crate::pushover::payload::Payload {
        crate::pushover::payload::Payload {
            token: "token".to_owned(),
            user: "user".to_owned(),
            message: "message".to_owned(),
        }
    }

    #[async_std::test]
    async fn missing_message() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_raw(stubs::missing_message(), "application/json"),
            )
            .mount(&mock_server)
            .await;

        let result = send_request(&mock_server.uri(), payload());

        assert!(
            matches!(result, Err(PushoverError::ApiError(t)) if t.errors == vec!["message cannot be blank"])
        );
    }

    #[async_std::test]
    async fn invalid_user() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(400).set_body_raw(stubs::invalid_user(), "application/json"),
            )
            .mount(&mock_server)
            .await;

        let result = send_request(&mock_server.uri(), payload());

        assert!(
            matches!(result, Err(PushoverError::ApiError(t)) if t.errors == vec!["user identifier is not a valid user, group, or subscribed user key"])
        );
    }

    #[async_std::test]
    async fn invalid_token() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(400).set_body_raw(stubs::invalid_token(), "application/json"),
            )
            .mount(&mock_server)
            .await;

        let result = send_request(&mock_server.uri(), payload());

        assert!(
            matches!(result, Err(PushoverError::ApiError(t)) if t.errors == vec!["application token is invalid"])
        );
    }

    #[async_std::test]
    async fn five_hundred_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let result = send_request(&mock_server.uri(), payload());

        assert!(matches!(result, Err(PushoverError::HttpError(_))));
    }
}
