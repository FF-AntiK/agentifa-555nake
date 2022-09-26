use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Segment {
    pub synced: Property<bool>,
}

impl Segment {
    pub fn new() -> Self {
        return Segment::new_complete(false);
    }
}
