use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

/// App State with the connection pool
struct AppState {
    pool: PgPool,
}

/// Health check endpoint to test database connection
async fn health_check(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query("SELECT 1")
        .execute(&data.pool)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Database connection is healthy!"),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Database connection failed")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Create a connection pool
    let pool: PgPool = PgPool::connect_lazy(&database_url)
        .expect("Failed to create database pool");

    // Start the Actix-web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(middleware::Logger::default()) // Logger middleware
            .route("/health", web::get().to(health_check)) // Health check endpoint
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
