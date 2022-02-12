use std::fmt;

use super::response_body::ResponseBody;

#[derive(Debug)]
pub enum PushoverError {
    HttpError(reqwest::Error),
    ApiError(ResponseBody),
}

impl std::error::Error for PushoverError {}

impl fmt::Display for PushoverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PushoverError::HttpError(_) => write!(f, "HTTP Error"),
            PushoverError::ApiError(_) => write!(f, "API Error"),
        }
    }
}

impl From<reqwest::Error> for PushoverError {
    fn from(original_error: reqwest::Error) -> Self {
        PushoverError::HttpError(original_error)
    }
}
