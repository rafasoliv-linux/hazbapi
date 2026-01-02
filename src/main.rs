// Mods
mod app;
mod conn;
mod routes;


#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        println!("Error to init API - Error: {}", e)
    }
}
