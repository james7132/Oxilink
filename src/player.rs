use songbird::{
    Call,
    id::{UserId, GuildId},
    input::Input,
    tracks::TrackHandle
};

#[derive(Clone)]
pub(crate) struct Player {
    call: Call,
    currently_playing: Option<TrackHandle>
}

impl Player {

    pub fn new(guild_id: GuildId, user_id: UserId) -> Self {
        Self {
            call: Call::standalone(guild_id, user_id),
            currently_playing: None
        }
    }

    pub async fn connect(&mut self, info: songbird::ConnectionInfo) {
        self.call.connect(info).await;
    }

    pub fn play(&mut self, source: Input) {
        self.currently_playing = Some(self.call.play_only_source(source));
    }

    pub fn stop(&mut self) {
        self.call.stop();
    }

    pub fn set_volume(&mut self, volume: u16) {
    }

    pub fn set_band_gain(&mut self, band: u8, gain: f32) {
    }

    pub fn set_paused(&mut self, paused: bool) {
    }

    pub fn seek_to(&mut self, position: u64) {
    }

}
