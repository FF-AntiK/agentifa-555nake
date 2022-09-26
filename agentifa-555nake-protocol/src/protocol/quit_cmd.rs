use bevy_ecs::prelude::Component;
use naia_shared::Replicate;

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct QuitCmd;

impl QuitCmd {
    pub fn new() -> Self {
        QuitCmd::new_complete()
    }
}
