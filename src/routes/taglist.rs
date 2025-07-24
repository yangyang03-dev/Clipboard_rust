use axum::{
    extract::{Path, State},
    routing::{get, post, put, patch, delete},
    Json, Router,
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use crate::models::taglist::{TagList, TagItem};

pub type TagListStore = Arc<Mutex<Vec<TagList>>>;

pub fn taglist_routes(store: TagListStore) -> Router {
    Router::new()
        .route("/taglists", get(list_taglists).post(create_taglist))
        .route("/taglists/:id", get(get_taglist).put(update_taglist).delete(delete_taglist))
        .route("/taglists/:id/items", patch(add_items))
        .route("/taglists/:id/items/:index", delete(delete_item))
        .with_state(store)
}

async fn list_taglists(State(store): State<TagListStore>) -> Json<Vec<TagList>> {
    Json(store.lock().unwrap().clone())
}

async fn create_taglist(State(store): State<TagListStore>, Json(mut payload): Json<TagList>) -> (StatusCode, Json<TagList>) {
    payload.id = uuid::Uuid::new_v4().to_string();
    store.lock().unwrap().push(payload.clone());
    (StatusCode::CREATED, Json(payload))
}

async fn get_taglist(Path(id): Path<String>, State(store): State<TagListStore>) -> Result<Json<TagList>, StatusCode> {
    store
        .lock()
        .unwrap()
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_taglist(Path(id): Path<String>, State(store): State<TagListStore>, Json(updated): Json<TagList>) -> Result<StatusCode, StatusCode> {
    let mut store = store.lock().unwrap();
    if let Some(existing) = store.iter_mut().find(|t| t.id == id) {
        *existing = updated;
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_taglist(Path(id): Path<String>, State(store): State<TagListStore>) -> StatusCode {
    let mut store = store.lock().unwrap();
    let len_before = store.len();
    store.retain(|t| t.id != id);
    if store.len() < len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn add_items(Path(id): Path<String>, State(store): State<TagListStore>, Json(new_items): Json<Vec<TagItem>>) -> Result<StatusCode, StatusCode> {
    let mut store = store.lock().unwrap();
    if let Some(taglist) = store.iter_mut().find(|t| t.id == id) {
        taglist.items.extend(new_items);
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_item(Path((id, index)): Path<(String, usize)>, State(store): State<TagListStore>) -> StatusCode {
    let mut store = store.lock().unwrap();
    if let Some(taglist) = store.iter_mut().find(|t| t.id == id) {
        if index < taglist.items.len() {
            taglist.items.remove(index);
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}