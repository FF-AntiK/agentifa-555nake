use std::f32::consts::PI;

use bevy_ecs::prelude::Component;
use naia_shared::{derive_serde, serde, Property, Replicate};
use rand::{distributions::Standard, prelude::Distribution};

#[derive(Copy)]
#[derive_serde]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..3) {
            0 => Direction::Down,
            1 => Direction::Left,
            2 => Direction::Right,
            _ => Direction::Up,
        }
    }
}

impl Direction {
    pub fn angle(self) -> f32 {
        PI * match self {
            Direction::Left => 0.0,
            Direction::Right => 1.0,
            _ => 0.5,
        }
    }

    pub fn flip_x(self) -> bool {
        match self {
            Direction::Up => true,
            _ => false,
        }
    }

    pub fn flip_y(self) -> bool {
        match self {
            Direction::Right => true,
            _ => false,
        }
    }
}

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Head {
    pub dir: Property<Direction>,
    pub name: Property<String>,
    pub running: Property<bool>,
}

impl Head {
    pub fn new(name: String) -> Self {
        return Head::new_complete(Direction::Up, name, false);
    }
}
