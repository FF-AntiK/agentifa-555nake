use bevy_ecs::prelude::Component;
use naia_derive::Replicate;
use naia_shared::Property;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct HighScore {
    pub name: Property<String>,
    pub score: Property<usize>,
}

impl HighScore {
    pub fn new(name: String, score: usize) -> Self {
        return HighScore::new_complete(name, score);
    }
}
