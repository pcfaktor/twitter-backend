use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::following;

pub async fn toggle_following(
    State(pool): State<PgPool>,
    Json(input): Json<following::ToggleFollowing>,
) -> Result<Json<following::FollowingResult>, StatusCode> {
    following::toggle(&pool, input)
        .await
        .map(|is_following| Json(following::FollowingResult { is_following }))
        .map_err(|_| StatusCode::BAD_REQUEST)
}
