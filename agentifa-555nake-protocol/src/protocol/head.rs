use bevy_ecs::prelude::Component;
use naia_shared::{derive_serde, serde, Property, Replicate};

#[derive_serde]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Head {
    pub dir: Property<Direction>,
    pub name: Property<String>,
}

impl Head {
    pub fn new(dir: Direction, name: String) -> Self {
        return Head::new_complete(dir, name);
    }
}
