use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Name {
    pub text: Property<String>,
}

impl Name {
    pub fn new(text: String) -> Self {
        return Name::new_complete(text);
    }
}
