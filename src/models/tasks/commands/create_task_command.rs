use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTaskCommand {
    pub url: String,
    pub http_method: String,
    pub repetition_seconds: Option<u32>
}
