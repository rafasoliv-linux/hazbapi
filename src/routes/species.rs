// External imports/USEs
use axum::{
    Router,
    Extension,
    Json,
    routing::get,
    http   
};
use serde::Serialize;

// Internal imports/USEs
use crate::conn::AppState;

#[derive(Serialize)]
struct Character {
    name: String,
    data: serde_json::Value
}

async fn get_all_angels(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let angels: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data FROM characters WHERE species = 'angel'"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(angels))
}

async fn get_all_demons(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let demons: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data FROM characters WHERE species = 'demon'"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(demons))
}

pub fn router() -> Router {
    Router::new()
        .route("/angels", get(get_all_angels))
        .route("/demons", get(get_all_demons))
}
