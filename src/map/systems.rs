use super::bundles::*;
use super::components::{Hex, TerrainType};
use super::events::*;
use super::resources::*;
use crate::input;
use bevy::prelude::*;

pub fn remove_tile(
    _trigger: On<input::events::RemoveTile>,
    mut tiles_by_hex: ResMut<TilesByHex>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    if let Some(new_tile) = tiles_by_hex.entities.remove(&hex) {
        commands.entity(new_tile).despawn();
    }
    commands.trigger(TilesChanged);
}

pub fn place_tile(
    _trigger: On<input::events::PlaceTile>,
    mut tiles_by_hex: ResMut<TilesByHex>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    let new_tile = commands
        .spawn(TileBundle::new(hex, TerrainType::Grass))
        .id();

    // Replace an already existing tile, instead of leaking it
    if let Some(old_tile) = tiles_by_hex.entities.insert(hex, new_tile) {
        commands.entity(old_tile).despawn();
    }
    commands.trigger(TilesChanged);
}
