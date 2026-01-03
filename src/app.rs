// Eternal imports/USEs
use axum::{Router, Extension, routing::get};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

// Internal imports/USEs
use crate::routes::{
    characters,
    species,
    weapons
};
use crate::conn::{AppState, create_pool};

async fn create_app() -> Router {
    let pool: PgPool = create_pool().await.expect("Failed to create pool");
    let state: AppState = AppState { pool: pool }; 
    Router::new()
        .nest("/characters",  characters::router())
        .nest("/species", species::router())
        .nest("/weapons", weapons::router())
        .route("/ping", get(|| async { "Pong" }))
        .layer(Extension(state))
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router = create_app().await;
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{}", port);
    let listener: TcpListener = TcpListener::bind(addr)
        .await?;

    axum::serve(listener, app)
        .await?;
    Ok(())
}
