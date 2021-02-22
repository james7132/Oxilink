mod models;

use crate::player::Player;
use std::collections::HashMap;
use models::ClientMessage;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use songbird::id::{UserId, GuildId};

pub(crate) struct OxilinkWs  {
    pub user_id: UserId,
    pub shard_count: u64,
    players: HashMap<GuildId, Player>
}

impl OxilinkWs {

    pub fn new(user_id: UserId, shard_count: u64) -> Self {
        Self {
            user_id: user_id,
            shard_count: shard_count,
            players: HashMap::new()
        }
    }

    fn get_player(&mut self, guild_id: u64) -> &Player {
        let id = GuildId(guild_id);
        if !self.players.contains_key(&id) {
            self.players.insert(id, Player::new(id, self.user_id));
        }
        self.players.get_mut(&id).unwrap()
    }

    fn get_player_mut(&mut self, guild_id: u64) -> &mut Player {
        let id = GuildId(guild_id);
        if !self.players.contains_key(&id) {
            self.players.insert(id, Player::new(id, self.user_id));
        }
        self.players.get_mut(&id).unwrap()
    }

    fn destroy_player(&mut self, guild_id: u64) {
        self.players.remove(&GuildId(guild_id));
    }

    fn handle_text(&mut self, raw_text: &str) -> Result<(), serde_json::Error> {
        match serde_json::from_str(raw_text)? {
            ClientMessage::VoiceUpdate(evt) => self.handle_voice_update(evt),
            ClientMessage::PlayTrack(evt) => self.handle_play(evt),
            ClientMessage::Stop(evt) => self.get_player_mut(evt.guild_id).stop(),
            ClientMessage::Seek(evt) =>
                self.get_player_mut(evt.guild_id).seek_to(evt.position),
            ClientMessage::SetVolume(evt) =>
                self.get_player_mut(evt.guild_id).set_volume(evt.volume),
            ClientMessage::SetEq(evt) => self.handle_set_eq(evt),
            ClientMessage::Destroy(evt) => self.destroy_player(evt.guild_id),
        }
        Ok(())
    }

    fn handle_voice_update(&mut self, evt: models::VoiceUpdate) {
        // discord sometimes send a partial server update missing the endpoint, which can be
        // ignored.
        if let Some(endpoint) = evt.event.endpoint {
            let connection_info = songbird::ConnectionInfo {
                endpoint: endpoint,
                user_id: self.user_id,
                session_id: evt.session_id,
                token: evt.event.token,
                guild_id: GuildId(evt.guild_id),
            };

            let fut = self.get_player_mut(evt.guild_id)
                          .connect(connection_info);
        }
    }

    fn handle_play(&mut self, evt: models::PlayTrack) {
        // TODO(james7132): Implement
        self.get_player(evt.guild_id);
    }

    fn handle_seek(&mut self, evt: models::Seek) {
        // TODO(james7132): Implement
        self.get_player(evt.guild_id);
    }

    fn handle_set_eq(&mut self, evt: models::SetEq) {
        let player = self.get_player_mut(evt.guild_id);
        for band in evt.bands.iter() {
            player.set_band_gain(band.band, band.gain);
        }
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
            Ok(ws::Message::Ping(msg)) => {ctx.pong(&msg);}
            Ok(ws::Message::Text(text)) => {self.handle_text(text.as_ref());},
            _ => (),
        }
    }
}
