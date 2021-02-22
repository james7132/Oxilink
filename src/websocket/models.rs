use serde::{Serialize, Deserialize};

// TODO(james7132): Properly implement a op-code based deserializer
#[derive(Debug, Deserialize)]
pub(super) enum ClientMessage {
    VoiceUpdate(VoiceUpdate),
    PlayTrack(PlayTrack),
    Stop(Stop),
    Seek(Seek),
    SetVolume(SetVolume),
    SetEq(SetEq),
    Destroy(Destroy)
}

pub(super) enum ClientResponse {
    Stats,
    PlayerUpdate(PlayerUpdate),
    PlayerEvent(PlayerEvent)
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct PlayerUpdate {
    pub guild_id: u64,
    pub state: serde_json::Value,
}

#[derive(Debug)]
pub(super) enum PlayerEvent {
    TrackStartEvent,
    TrackEndEvent,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TrackEndEvent {
    pub track: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TrackExceptionEvent {
    pub track: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TrackStuckEvent {
    pub track: String,
    #[serde(rename = "thresholdMs")]
    pub threshold_ms: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct VoiceUpdate {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
    pub session_id: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct PlayTrack {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
    pub track: String,
    pub start_time: u32,
    pub end_time: u32,
    pub volume: u16,
    pub no_replace: bool,
    pub pause: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Stop {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Seek {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
    pub position: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct SetVolume {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
    pub volume: u16
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct SetEq {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
    pub bands: Vec<EqBand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct EqBand {
    pub band: u8,
    pub gain: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Destroy {
    #[serde(alias = "guildId")]
    pub guild_id: u64,
}
