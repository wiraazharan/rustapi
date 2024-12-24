use actix_web::HttpMessage; // For extensions
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::{MySqlPool, FromRow, Row};

// Define a struct to represent the user
#[derive(FromRow, Debug)]
struct User {
    id: u64, // Use u64 for BIGINT UNSIGNED
    username: String,
}

pub async fn hello_world_handler(
    req: HttpRequest,
    db_pool: web::Data<MySqlPool>, // Access the database pool
) -> HttpResponse {
    // Retrieve the token from request extensions
    if let Some(token) = req.extensions().get::<String>() {
        // Query the database and retrieve the result
        let raw_result = sqlx::query("SELECT id, username FROM users WHERE id = ? LIMIT 1")
            .bind(token) // Bind the token value
            .fetch_optional(&**db_pool)
            .await;

        match raw_result {
            Ok(Some(row)) => {
                // Extract the raw row information
                let id: u64 = row.try_get::<u64, _>("id").unwrap_or_default(); // Explicitly specify u64
                let username: String = row.try_get("username").unwrap_or_default();

                // Use the extracted fields
                let user = User { id, username };

                // Return both structured data and raw data for debugging
                HttpResponse::Ok().json(serde_json::json!({
                    "message": format!("Welcome, {}!", user.username),
                    "raw": {
                        "id": user.id,
                        "username": user.username
                    }
                }))
            }
            Ok(None) => {
                // Token not found
                HttpResponse::Unauthorized().body("Invalid token")
            }
            Err(err) => {
                // Database error
                eprintln!("Database error: {:?}", err);
                HttpResponse::InternalServerError().body("Database query error")
            }
        }
    } else {
        HttpResponse::BadRequest().body("Token is missing")
    }
}