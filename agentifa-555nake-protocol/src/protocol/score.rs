use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Score {
    pub level: Property<usize>,
}

impl Score {
    pub fn new() -> Self {
        return Score::new_complete(0);
    }
}
