mod models;

use crate::player::Player;
use dashmap::DashMap;
use models::ClientMessage;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use songbird::id::{UserId, GuildId};

pub(crate) struct OxilinkWs  {
    pub user_id: UserId,
    pub shard_count: u64,
    players: DashMap<GuildId, Player>
}

impl OxilinkWs {

    pub fn new(user_id: UserId, shard_count: u64) -> Self {
        Self {
            user_id: user_id,
            shard_count: shard_count,
            players: DashMap::new()
        }
    }

    fn create_player(&self, guild_id: GuildId) -> Player {
        let player = Player::new(guild_id, self.user_id);
        self.players.insert(guild_id, player.clone());
        player
    }

    fn get_player(&self, guild_id: GuildId) -> Option<Player> {
        self.players.get(&guild_id).map(|kv| kv.value().clone())
    }

    fn destroy_player(&self, guild_id: GuildId) {
        self.players.remove(&guild_id);
    }

    fn handle_text(&self, raw_text: &str) -> Result<(), serde_json::Error> {
        match serde_json::from_str(raw_text)? {
            ClientMessage::VoiceUpdate(evt) => self.handle_voice_update(evt),
            ClientMessage::PlayTrack(evt) => self.handle_play(evt),
            ClientMessage::Stop(evt) => self.handle_stop(evt),
            ClientMessage::Seek(evt) => self.handle_seek(evt),
            ClientMessage::SetVolume(evt) => self.handle_set_volume(evt),
            ClientMessage::SetEq(evt) => self.handle_set_eq(evt),
            ClientMessage::Destroy(evt) => self.destroy_player(GuildId(evt.guild_id)),
        }
        Ok(())
    }

    fn handle_voice_update(&self, evt: models::VoiceUpdate) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

    fn handle_play(&self, evt: models::PlayTrack) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

    fn handle_stop(&self, evt: models::Stop) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

    fn handle_seek(&self, evt: models::Seek) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

    fn handle_set_volume(&self, evt: models::SetVolume) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

    fn handle_set_eq(&self, evt: models::SetEq) {
        // TODO(james7132): Implement
        self.get_player(GuildId(evt.guild_id));
    }

}

impl Actor for OxilinkWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for OxilinkWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.ping(&msg),
            Ok(ws::Message::Text(text)) => {
                self.handle_text(text.as_ref());
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
