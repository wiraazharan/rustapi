use chrono::{Utc, Duration};
use crate::utils::jwt_utils::generate_jwt;

pub fn refresh_token(user_id: i32, secret: &str) -> String {
    let expiration = (Utc::now() + Duration::days(1)).timestamp() as usize;
    generate_jwt(user_id, secret, expiration)
}
