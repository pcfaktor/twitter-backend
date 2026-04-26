use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use crate::handlers::{
    create_tweet, create_user, delete_tweet, get_feed, get_tweets, get_user, get_user_tweets,
    get_users, toggle_following,
};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/{username}", get(get_user))
        .route("/users/{username}/tweets", get(get_user_tweets))
        .route("/tweets", get(get_tweets).post(create_tweet))
        .route("/tweets/{id}", delete(delete_tweet))
        .route("/tweets/feed/{user_id}", get(get_feed))
        .route("/following/toggle", post(toggle_following))
        .layer(CorsLayer::permissive())
        .with_state(pool)
}
