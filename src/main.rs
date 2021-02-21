mod websocket;
mod player;

use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dashmap::DashMap;
use std::sync::Arc;

type GuildId = u64;

struct AppStateRef {
    pub players: DashMap<GuildId, player::Player>
}

#[derive(Clone)]
pub(crate) struct AppState(Arc<AppStateRef>);

impl AppState {

    pub fn new() -> Self {
        Self(Arc::new(AppStateRef {
            players: DashMap::new()
        }))
    }

    fn create_player(&self, guild_id: GuildId) -> player::Player {
        let player = player::Player::new(guild_id);
        self.0.players.insert(guild_id, player.clone());
        player
    }

    fn get_player(&self, guild_id: GuildId) -> Option<player::Player> {
        self.0.players.get(&guild_id).map(|kv| (*kv.value()).clone())
    }

    fn destroy_player(&self, guild_id: GuildId) {
        self.0.players.remove(&guild_id);
    }

}

#[get("/ws/")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>) -> Result<HttpResponse, Error>
{
    let resp = ws::start(websocket::OxilinkWs::new(state.get_ref().clone()), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new();
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
