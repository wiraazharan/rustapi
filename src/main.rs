use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod config;
mod controllers;
mod middleware;
mod routes; // Add this for database configuration

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize database pool
    let db_pool = config::database::initialize_db_pool().await;

    // Retrieve port from environment variables or set default
    let port = env::var("PORT").unwrap_or_else(|_| "8003".to_string());

    println!("Starting server on 127.0.0.1:{}", port);

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // Share the DB pool across the app
            .wrap(middleware::token_extractor::TokenExtractor) // Add request middleware
            .wrap(middleware::response_processor::ResponseProcessor) // Add response middleware
            .configure(routes::testendpoints::configure_routes) // Configure routes
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
