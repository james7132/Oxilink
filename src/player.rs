use std::sync::Arc;

struct PlayerRef {
    guild_id: u64
}

#[derive(Clone)]
pub(crate) struct Player(Arc<PlayerRef>);

impl Player {

    pub fn new(guild_id: u64) -> Self {
        Self(Arc::new(PlayerRef {
            guild_id: guild_id
        }))
    }

}
