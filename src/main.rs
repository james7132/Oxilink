mod websocket;
mod player;
mod error;

use actix_web::{
    web, get, App, HttpRequest, Responder, HttpServer,
    http::header::HeaderMap,
};
use std::str::FromStr;
use actix_web_actors::ws;
use error::HeaderError;

fn get_header_as<T: FromStr>(headers: &HeaderMap, header: &str) -> Result<T, HeaderError> {
    headers.get(header)
        .ok_or_else(|| HeaderError::Missing(header.to_owned()))
        .and_then(|id| id.to_str().map_err(|_| HeaderError::Missing(header.to_owned())))
        .and_then(|id| id.parse::<T>().map_err(|_| HeaderError::Parse(header.to_owned())))
}

#[get("/ws/")]
async fn index(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let headers = req.headers();
    let auth = get_header_as::<String>(&headers, "Authorization")?;
    let user_id = songbird::id::UserId(get_header_as(&headers, "User-Id")?);
    let shard_count = get_header_as::<u64>(&headers, "Num-Shards")?;

    let context = websocket::OxilinkWs::new(user_id, shard_count);
    ws::start(context, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
