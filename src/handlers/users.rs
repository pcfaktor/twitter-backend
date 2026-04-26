use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::user;

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<user::User>>, StatusCode> {
    user::get_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<user::User>, StatusCode> {
    user::get_by_username(&pool, &username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(input): Json<user::CreateUser>,
) -> Result<(StatusCode, Json<user::User>), StatusCode> {
    user::create(&pool, input)
        .await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(|_| StatusCode::BAD_REQUEST)
}