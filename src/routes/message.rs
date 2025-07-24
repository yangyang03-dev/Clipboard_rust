use axum::{
    extract::{Path, State},
    Json, Router, routing::{get, post, put, delete},
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use crate::models::message::Message;

pub type MessageStore = Arc<Mutex<Vec<Message>>>;

pub fn message_routes(store: MessageStore) -> Router {
    Router::new()
        .route("/messages", get(list_messages).post(create_message))
        .route("/messages/:id", get(get_message).put(update_message).delete(delete_message))
        .with_state(store)
}

async fn list_messages(State(store): State<MessageStore>) -> Json<Vec<Message>> {
    let store = store.lock().unwrap();
    Json(store.clone())
}

async fn create_message(
    State(store): State<MessageStore>,
    Json(payload): Json<Message>,
) -> (StatusCode, Json<Message>) {
    let mut store = store.lock().unwrap();
    store.push(payload.clone());
    (StatusCode::CREATED, Json(payload))
}

async fn get_message(
    Path(id): Path<String>,
    State(store): State<MessageStore>,
) -> Result<Json<Message>, StatusCode> {
    let store = store.lock().unwrap();
    store.iter().find(|m| m.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_message(
    Path(id): Path<String>,
    State(store): State<MessageStore>,
    Json(updated): Json<Message>,
) -> Result<StatusCode, StatusCode> {
    let mut store = store.lock().unwrap();
    if let Some(m) = store.iter_mut().find(|m| m.id == id) {
        *m = updated;
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_message(
    Path(id): Path<String>,
    State(store): State<MessageStore>,
) -> StatusCode {
    let mut store = store.lock().unwrap();
    let len_before = store.len();
    store.retain(|m| m.id != id);
    if store.len() < len_before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}