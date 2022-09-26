use std::fmt::Display;

use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Property, Replicate};
use rand::random;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Position {
    pub entity: EntityProperty,
    pub x: Property<usize>,
    pub y: Property<usize>,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({};{})", *self.x, *self.y)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position::new_complete(x, y)
    }

    pub fn rnd(grid_size: usize) -> Self {
        Self::new(random::<usize>() % grid_size, random::<usize>() % grid_size)
    }
}
