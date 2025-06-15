use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueueResponse {
    pub queued: bool,
    pub message: String,
    pub cancel_token: String,
}

impl QueueResponse {
    pub fn new(q: bool, m: &str, t: &str) -> Self {
        Self {
            queued: q,
            message: m.to_string(),
            cancel_token: t.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrResponse {
    pub error_code: i32,
    pub message: String,
}

impl ErrResponse {
    pub fn new(e: i32, m: &str) -> Self {
        Self {
            error_code: e,
            message: m.to_string(),
        }
    }   
}