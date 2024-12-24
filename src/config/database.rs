use dotenvy::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

pub async fn initialize_db_pool() -> MySqlPool {
    // Load environment variables
    dotenv().ok();

    // Retrieve database credentials from .env
    let db_host = env::var("DB_HOST").expect("DB_HOST not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT not set");
    let db_name = env::var("DB_DATABASE").expect("DB_DATABASE not set");
    let db_user = env::var("DB_USERNAME").expect("DB_USERNAME not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");

    // Build the database URL
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    // Create and return a connection pool
    MySqlPoolOptions::new()
        .max_connections(5) // Adjust as needed
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}