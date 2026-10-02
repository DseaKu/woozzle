use bevy::prelude::*;

pub mod components;
pub mod conversion;

pub struct HexPlugin;
impl Plugin for HexPlugin {
    fn build(&self, _app: &mut App) {}
}
