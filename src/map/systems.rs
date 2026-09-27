use super::bundles::*;
use super::components::{Hex, TerrainType};
use super::events::*;
use super::resources::*;
use crate::camera;
use crate::input;
use bevy::prelude::*;


pub fn update_visible_tiles<E: Event>(
    _trigger: On<E>,
    visible_hexes: Res<camera::resources::VisibleHexes>,
    mut visible_tiles: ResMut<VisibleTiles>,
    tile_entities: Res<TileEntities>,
    mut commands: Commands,
) {
    visible_tiles.entities.clear();
    for hex in &visible_hexes.tiles {
        if let Some(visible_tile) = tile_entities.entities.get(hex) {
            visible_tiles.entities.push(*visible_tile);
        }
    }
    commands.trigger(VisibleUpdated);
}

pub fn remove_tiles(
    _trigger: On<input::events::RemoveTile>,
    mut tile_entities: ResMut<TileEntities>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    tile_entities.entities.remove(&hex);
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

    tile_entities.entities.insert(hex, tile_entity);
    commands.trigger(DataUpdated);
}

