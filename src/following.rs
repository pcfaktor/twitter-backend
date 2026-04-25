use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ToggleFollowing {
    pub follower_id: Uuid,
    pub following_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct FollowingResult {
    pub is_following: bool,
}

pub async fn toggle(pool: &PgPool, input: ToggleFollowing) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        WITH deleted AS (
            DELETE FROM following
            WHERE follower_id = $1 AND following_id = $2
            RETURNING 1
        ),
        inserted AS (
            INSERT INTO following (follower_id, following_id)
            SELECT $1, $2
            WHERE NOT EXISTS (SELECT 1 FROM deleted)
            RETURNING 1
        )
        SELECT EXISTS (SELECT 1 FROM inserted) AS "is_following!"
        "#,
        input.follower_id,
        input.following_id,
    )
    .fetch_one(pool)
    .await
}
