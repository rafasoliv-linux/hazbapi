// External imports/USEs
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenv::dotenv;
use std::env;

// Internal imports/USEs

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool
}

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL.as_str())
        .await?;
    Ok(pool)
}
