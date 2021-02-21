mod models;

use crate::AppState;
use models::ClientMessage;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

pub(crate) struct OxilinkWs  {
    state: AppState,
}

impl OxilinkWs {

    pub fn new(state: AppState) -> Self {
        Self { state: state }
    }

    fn handle_text(&self, raw_text: &str) -> Result<(), serde_json::Error> {
         self.handle_message(serde_json::from_str(raw_text)?);
         Ok(())
    }

    fn handle_message(&self, message: ClientMessage) {
        match message {
            ClientMessage::VoiceUpdate(evt) => self.handle_voice_update(evt),
            ClientMessage::PlayTrack(evt) => self.handle_play(evt),
            ClientMessage::Stop(evt) => self.handle_stop(evt),
            ClientMessage::Seek(evt) => self.handle_seek(evt),
            ClientMessage::SetVolume(evt) => self.handle_set_volume(evt),
            ClientMessage::SetEq(evt) => self.handle_set_eq(evt),
            ClientMessage::Destroy(evt) => self.handle_destroy(evt),
        }
    }

    fn handle_voice_update(&self, evt: models::VoiceUpdate) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_play(&self, evt: models::PlayTrack) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_stop(&self, evt: models::Stop) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_seek(&self, evt: models::Seek) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_set_volume(&self, evt: models::SetVolume) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_set_eq(&self, evt: models::SetEq) {
        // TODO(james7132): Implement
        self.state.get_player(evt.guild_id);
    }

    fn handle_destroy(&self, evt: models::Destroy) {
        self.state.destroy_player(evt.guild_id);
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
