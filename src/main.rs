use actix_files::Files;
use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("Failed to parse PORT variable");

    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Build the path to the "static" folder
    let static_dir = current_dir.join("static");

    HttpServer::new(move || {
        App::new().service(Files::new("/", static_dir.clone()).index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
