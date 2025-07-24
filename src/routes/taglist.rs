use axum::{
    extract::{Path, State},
    routing::{get, post, put, patch, delete},
    Json, Router,
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::taglist::{TagList, TagItem};

pub fn taglist_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/taglists", get(list_taglists).post(create_taglist))
        .route("/taglists/:id", get(get_taglist).put(update_taglist).delete(delete_taglist))
        .route("/taglists/:id/items", patch(add_items))
        .route("/taglists/:id/items/:index", delete(delete_item))
        .with_state(pool)
}

async fn list_taglists(State(pool): State<PgPool>) -> Result<Json<Vec<TagList>>, StatusCode> {
    let taglists = sqlx::query_as!(TagList, "SELECT id, name, created_at FROM taglists ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(taglists))
}

async fn create_taglist(State(pool): State<PgPool>, Json(payload): Json<TagList>) -> Result<(StatusCode, Json<TagList>), StatusCode> {
    let id = Uuid::new_v4();
    let name = payload.name;

    let taglist = sqlx::query_as!(
    TagList,
    "INSERT INTO taglists (id, name) VALUES ($1, $2) RETURNING id, name, created_at",
    id,
    name
)
.fetch_one(&pool)
.await
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(taglist)))
}

async fn get_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<Json<TagList>, StatusCode> {
    let taglist = sqlx::query_as!(TagList, "SELECT id, name, created_at FROM taglists WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(taglist))
}

async fn update_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>, Json(updated): Json<TagList>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("UPDATE taglists SET name = $1 WHERE id = $2", updated.name, id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}

async fn delete_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> StatusCode {
    let result = sqlx::query!("DELETE FROM taglists WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        Ok(_) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn add_items(Path(id): Path<Uuid>, State(pool): State<PgPool>, Json(new_items): Json<Vec<TagItem>>) -> Result<StatusCode, StatusCode> {
    for item in new_items {
        sqlx::query!(
            "INSERT INTO tag_items (id, taglist_id, tag, remark) VALUES ($1, $2, $3, $4)",
            Uuid::new_v4(),
            id,
            item.tag,
            item.remark
        )
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(StatusCode::OK)
}

async fn delete_item(Path((id, index)): Path<(Uuid, usize)>, State(pool): State<PgPool>) -> StatusCode {
    // You may want to change this logic to use actual item id
    // Here we simulate index-based delete (not ideal in SQL)
    if let Ok(items) = sqlx::query_as!(TagItem, "SELECT * FROM tag_items WHERE taglist_id = $1", id)
        .fetch_all(&pool)
        .await
    {
        if let Some(item) = items.get(index) {
            let _ = sqlx::query!("DELETE FROM tag_items WHERE id = $1", item.id)
                .execute(&pool)
                .await;
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}