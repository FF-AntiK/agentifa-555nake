use bevy_ecs::prelude::Component;
use naia_derive::Replicate;
use naia_shared::Property;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Auth {
    pub key: Property<String>,
}

impl Auth {
    pub fn new(key: &str) -> Self {
        return Auth::new_complete(key.to_string());
    }
}
