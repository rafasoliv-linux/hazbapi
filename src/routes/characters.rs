use axum::{
    Router,
    Extension,
    Json,
    routing::get,
    extract::Path,
    http,
    };
use serde::Serialize;
// Internal imports/USEs
use crate::conn::AppState;

struct CharacterData {
    data: serde_json::Value
}
async fn get_character(Extension(state): Extension<AppState>, Path(character_name): Path<String>) -> Result<Json<serde_json::Value>, http::StatusCode> {
    let row: Option<CharacterData> = sqlx::query_as!(
        CharacterData,
        "SELECT data FROM characters WHERE name = $1",
        character_name
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(ch) => Ok(Json(ch.data)),
        None => Err(http::StatusCode::NOT_FOUND),
    }
}

#[derive(Serialize)]
struct Character {
    name: String,
    data: serde_json::Value
}

async fn get_all_characters(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data FROM characters"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(characters))
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_characters))
        .route("/{character_name}", get(get_character))
}
