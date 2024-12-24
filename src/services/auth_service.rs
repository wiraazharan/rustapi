use argon2::{self, Config};
use sqlx::MySqlPool;

pub async fn authenticate_user(username: &str, password: &str, db_pool: &MySqlPool) -> Option<i32> {
    let user = sqlx::query!("SELECT id, hashed_password FROM users WHERE username = ?", username)
        .fetch_optional(db_pool)
        .await
        .ok()?;

    if let Some(user) = user {
        if argon2::verify_encoded(&user.hashed_password, password.as_bytes()).unwrap_or(false) {
            return Some(user.id);
        }
    }
    None
}