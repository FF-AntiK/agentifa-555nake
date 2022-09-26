use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

use super::Direction;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct DirCmd {
    pub dir: Property<Direction>,
}

impl DirCmd {
    pub fn new(dir: Direction) -> Self {
        DirCmd::new_complete(dir)
    }
}
