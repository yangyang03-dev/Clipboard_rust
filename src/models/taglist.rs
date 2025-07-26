use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagList {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagItem {
    pub id: Uuid,
    pub taglist_id: Uuid,
    pub tag: String,
    pub remark: Option<String>,
}
#[derive(Deserialize)]
pub struct NewTagItem {
    pub tag: String,
    pub remark: Option<String>,
}