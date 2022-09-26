use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Vincible;

impl Vincible {
    pub fn new() -> Self {
        return Vincible::new_complete();
    }
}
