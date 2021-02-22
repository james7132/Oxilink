mod rest;
mod websocket;
mod player;
mod error;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().configure(rest::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
