use axum::{
    extract::{Path, State},
    routing::{get, post, put, patch, delete},
    Json, Router,
    http::StatusCode
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::taglist::{TagList, TagItem};

pub fn taglist_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/taglists", get(get_all_taglists).post(create_taglist))
        .route("/taglists/:id", get(get_taglist).put(update_taglist).delete(delete_taglist))
        .route("/taglists/:id/items", patch(add_items).get(get_items))
        .route("/taglists/:id/items/:item_id", delete(delete_item))
        .with_state(pool)
}

async fn get_all_taglists(State(pool): State<PgPool>) -> Result<Json<Vec<TagList>>, StatusCode> {
    let taglists = sqlx::query_as!(
        TagList,
        "SELECT id, name, created_at FROM taglists ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(taglists))
}

async fn create_taglist(State(pool): State<PgPool>, Json(input): Json<TagList>) -> Result<(StatusCode, Json<TagList>), StatusCode> {
    let id = Uuid::new_v4();
    let taglist = sqlx::query_as!(
        TagList,
        "INSERT INTO taglists (id, name) VALUES ($1, $2) RETURNING id, name, created_at",
        id,
        input.name
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(taglist)))
}

async fn get_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<Json<TagList>, StatusCode> {
    let taglist = sqlx::query_as!(
        TagList,
        "SELECT id, name, created_at FROM taglists WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    taglist.map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn update_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>, Json(updated): Json<TagList>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "UPDATE taglists SET name = $1 WHERE id = $2",
        updated.name,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::OK)
    }
}

async fn delete_taglist(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<StatusCode, StatusCode> {
    // Delete associated items first
    sqlx::query!("DELETE FROM tag_items WHERE taglist_id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query!("DELETE FROM taglists WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn add_items(Path(id): Path<Uuid>, State(pool): State<PgPool>, Json(items): Json<Vec<TagItem>>) -> Result<StatusCode, StatusCode> {
    for item in items {
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

async fn get_items(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<Json<Vec<TagItem>>, StatusCode> {
    let items = sqlx::query_as!(
        TagItem,
        "SELECT id, taglist_id, tag, remark FROM tag_items WHERE taglist_id = $1 ORDER BY id",
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

async fn delete_item(Path((id, item_id)): Path<(Uuid, Uuid)>, State(pool): State<PgPool>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM tag_items WHERE id = $1 AND taglist_id = $2",
        item_id,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}