use crate::websocket;
use super::utils;
use actix_web::{web, get, HttpRequest, Responder};
use actix_web_actors::ws;

#[get("/ws/")]
async fn websocket_entry(req: HttpRequest, stream: web::Payload) -> impl Responder {
    let headers = req.headers();

    let auth = utils::get_header_as::<String>(&headers, "Authorization")?;
    let user_id = songbird::id::UserId(utils::get_header_as(&headers, "User-Id")?);
    let shard_count = utils::get_header_as::<u64>(&headers, "Num-Shards")?;

    let context = websocket::OxilinkWs::new(user_id, shard_count);
    ws::start(context, &req, stream)
}

pub(super) fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(websocket_entry);
}
