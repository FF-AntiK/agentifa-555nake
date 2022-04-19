use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct HighScoreRank {
    pub position: Property<usize>,
    pub rank: Property<usize>,
}

impl HighScoreRank {
    pub fn new(position: usize, rank: usize) -> Self {
        return HighScoreRank::new_complete(position, rank);
    }
}
