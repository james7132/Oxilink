use std::sync::{Arc, RwLock};
use songbird::{
    Call,
    id::{UserId, GuildId},
    input::Input,
    tracks::TrackHandle
};

struct PlayerRef {
    call: Call,
    currently_playing: Option<TrackHandle>
}

#[derive(Clone)]
pub(crate) struct Player(Arc<RwLock<PlayerRef>>);

impl Player {

    pub fn new(guild_id: GuildId, user_id: UserId) -> Self {
        Self(Arc::new(RwLock::new(PlayerRef {
            call: Call::standalone(guild_id, user_id),
            currently_playing: None
        })))
    }

    pub fn play(&self, source: Input) {
        let mut player_ref = self.0.write().unwrap();
        player_ref.currently_playing = Some(player_ref.call.play_only_source(source));
    }

    pub fn stop(&self) {
        self.0.write().unwrap().call.stop();
    }

    pub fn set_paused(&self, paused: bool) {
    }

    pub fn seek_to(&self, position: u64) {
    }

}
