use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::tweet;

pub async fn get_tweets(State(pool): State<PgPool>) -> Result<Json<Vec<tweet::Tweet>>, StatusCode> {
    tweet::get_all(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user_tweets(
    State(pool): State<PgPool>,
    Path(username): Path<String>,
) -> Result<Json<Vec<tweet::Tweet>>, StatusCode> {
    tweet::get_by_username(&pool, &username)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_feed(
    State(pool): State<PgPool>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<Json<Vec<tweet::Tweet>>, StatusCode> {
    tweet::get_feed(&pool, user_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_tweet(
    State(pool): State<PgPool>,
    Json(input): Json<tweet::CreateTweet>,
) -> Result<(StatusCode, Json<tweet::Tweet>), StatusCode> {
    tweet::create(&pool, input)
        .await
        .map(|t| (StatusCode::CREATED, Json(t)))
        .map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn delete_tweet(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    tweet::delete(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(StatusCode::NOT_FOUND)
}
