use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub set_time: String, // ISO 8601 time string
}

impl Message {
    pub fn new(content: String, set_time: String) -> Self {
        Message {
            id: Uuid::new_v4().to_string(),
            content,
            set_time,
        }
    }
}