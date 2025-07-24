use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagItem {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagList {
    pub id: String,
    pub name: String,
    pub items: Vec<TagItem>,
}

impl TagList {
    pub fn new(name: String, items: Vec<TagItem>) -> Self {
        TagList {
            id: Uuid::new_v4().to_string(),
            name,
            items,
        }
    }
}