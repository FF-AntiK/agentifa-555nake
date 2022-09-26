use bevy_ecs::prelude::Component;
use naia_shared::{EntityProperty, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct AssignMsg {
    pub entity: EntityProperty,
}

impl AssignMsg {
    pub fn new() -> Self {
        AssignMsg::new_complete()
    }
}
