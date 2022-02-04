use std::fmt;

#[derive(Debug)]
pub enum PushoverError {
    HttpError,
    ApiError,
}

impl std::error::Error for PushoverError {}

impl fmt::Display for PushoverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PushoverError::HttpError => write!(f, "HTTP Error"),
            PushoverError::ApiError => write!(f, "API Error"),
        }
    }
}
