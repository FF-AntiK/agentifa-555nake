use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct StartCmd {
    pub name: Property<String>,
}

impl StartCmd {
    pub fn new(name: String) -> Self {
        StartCmd::new_complete(name)
    }
}
