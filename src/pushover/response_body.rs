use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseBody {
    #[serde(default = "default_message")]
    pub message: String,
    pub status: u8,
    pub request: String,
    #[serde(default = "default_errors")]
    pub errors: Vec<String>,
}

fn default_message() -> String {
    String::new()
}

fn default_errors() -> Vec<String> {
    Vec::new()
}
