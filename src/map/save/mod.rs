use bevy::prelude::*;

pub mod components;
mod systems;

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, _app: &mut App) {}
}
