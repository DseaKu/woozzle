use super::bundles::*;
use super::components::{Hex, TerrainType};
use super::events::*;
use super::resources::*;
use crate::input;
use bevy::prelude::*;

pub fn remove_tiles(
    _trigger: On<input::events::RemoveTile>,
    mut tile_entities: ResMut<TileEntities>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    if let Some(tile_entity) = tile_entities.entities.remove(&hex) {
        commands.entity(tile_entity).despawn();
    }
    commands.trigger(DataUpdated);
}

pub fn set_tile(
    _trigger: On<input::events::SetTile>,
    mut tile_entities: ResMut<TileEntities>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    let tile_entity = commands.spawn(HexTile::new(hex, TerrainType::Grass)).id();

    // Replace an already existing tile, instead of leaking it
    if let Some(old_tile_entity) = tile_entities.entities.insert(hex, tile_entity) {
        commands.entity(old_tile_entity).despawn();
    }
    commands.trigger(DataUpdated);
}
