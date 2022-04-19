use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Position {
    pub x: Property<usize>,
    pub y: Property<usize>,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        return Position::new_complete(x, y);
    }
}
