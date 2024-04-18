use actix_files::Files;
use actix_web::{App, HttpServer};
use std::env;

const DEFAULT_PORT: &str = "8000";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = "0.0.0.0";
    let port = env::var("PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse::<u16>()
        .expect("Failed to parse PORT variable");

    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get current directory");
    //t
    let static_dir = current_dir.join("static");
    //println!{"Hosting at: {}:{}", host, port};
    HttpServer::new(move || {
        App::new().service(Files::new("/", static_dir.clone()).index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
