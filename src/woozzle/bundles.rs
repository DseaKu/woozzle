use bevy::prelude::*;

#[derive(Bundle)]
pub struct Woozzle {
    transform: Transform,
}

impl Woozzle {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
}
