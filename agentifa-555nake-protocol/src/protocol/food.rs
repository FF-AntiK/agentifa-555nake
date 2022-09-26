use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Food {}

impl Food {
    pub fn new() -> Self {
        return Food::new_complete();
    }
}
