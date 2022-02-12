use serde::Serialize;

use crate::cli::Args;

#[derive(Debug, Serialize)]
pub struct Payload {
    pub token: String,
    pub user: String,
    pub message: String,
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
