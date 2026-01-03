// External imports/USEs
use axum::{
    Router,
    Json,
    Extension,
    routing::get,
    extract::Path,
    http
};
use serde::Serialize;

// Internal imports/USEs
use crate::conn::AppState;

#[derive(Serialize)]
struct Weapon {
    name: String,
    data: serde_json::Value
}

async fn get_all_weapons(Extension(state): Extension<AppState>) -> Result<Json<Vec<Weapon>>, http::StatusCode> {
    let weapons: Vec<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data FROM weapons"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(weapons))
}

async fn get_weapon(Extension(state): Extension<AppState>, Path(weapon_name): Path<String>) -> Result<Json<Weapon>, http::StatusCode> {
    let weapon: Option<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data FROM weapons WHERE name = $1",
        weapon_name
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match weapon {
        Some(weapon) => Ok(Json(weapon)),
        None => Err(http::StatusCode::NOT_FOUND)
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_weapons))
        .route("/{weapon_name}", get(get_weapon))
}
