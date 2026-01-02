// Eternal imports/USEs
use axum::{Router, Extension};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

// Internal imports/USEs
use crate::routes::{
    characters,
    species
};
use crate::conn::{AppState, create_pool};

async fn create_app() -> Router {
    let pool: PgPool = create_pool().await.expect("Failed to create pool");
    let state: AppState = AppState { pool: pool }; 
    Router::new()
        .nest("/characters",  characters::router())
        .nest("/species", species::router())
        .layer(Extension(state))
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router = create_app().await;
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000")
        .await?;

    axum::serve(listener, app)
        .await?;
    Ok(())
}
