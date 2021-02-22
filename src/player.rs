use songbird::{
    Call,
    id::{UserId, GuildId},
    input::Input,
    tracks::TrackHandle
};
use std::time::Duration;

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

    pub fn is_playing(&self) -> bool {
        self.currently_playing.is_some()
    }

    pub async fn connect(&mut self, info: songbird::ConnectionInfo) {
        self.call.connect(info).await;
    }

    pub fn play(&mut self, source: Input) {
        self.currently_playing = Some(self.call.play_only_source(source));
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.currently_playing.as_mut() {
            handle.stop();
        }
    }

    pub fn set_volume(&mut self, volume: u16) {
        if let Some(handle) = self.currently_playing.as_mut() {
            handle.set_volume(f32::from(volume) / 100.0);
        }
    }

    pub fn set_band_gain(&mut self, band: u8, gain: f32) {
        // TODO(james7132): Implement
    }

    pub fn set_paused(&mut self, paused: bool) {
        if let Some(handle) = self.currently_playing.as_mut() {
            if paused {
                handle.pause();
            } else {
                handle.play();
            }
            // TODO(james7132): Handle error here
        }
    }

    pub fn seek_to(&mut self, position: u64) {
        if let Some(handle) = self.currently_playing.as_mut() {
            if handle.is_seekable() {
                // TODO(james7132): Replicate the sample seeking
                handle.seek_time(Duration::from_millis(position));
            } else {
                // TODO(james7132): Handle error
            }
        }
    }

}
