// Mods
mod app;
mod conn;
mod routes;

// Internal imports/USEs
#[tokio::main]
async fn main() {

    if let Err(e) = app::run().await {
        println!("Fail to init API, Error: {}", e);
    }

}
