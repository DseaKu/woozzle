use super::components::{Hex, TerrainType};
use crate::camera;
use crate::input;
use bevy::prelude::*;

// 16 bit tile assets
const HEX_WIDTH: f32 = 9.25;
const HEX_HEIGHT: f32 = 8.0;

const SQRT_3_OVER_3: f32 = 0.577_350_26;
const SQRT_3: f32 = 1.732_050_8;
const TWO_THIRDS: f32 = 2.0 / 3.0;
const THREE_HALVES: f32 = 3.0 / 2.0;

pub fn update_visible_tiles<E: Event>(
    _trigger: On<E>,
    visible_hexes: Res<camera::resources::VisibleHexes>,
    mut visible_tiles: ResMut<super::resources::VisibleTiles>,
    tile_data: Res<super::resources::TileData>,
    mut commands: Commands,
) {
    visible_tiles.entities.clear();
    for hex in &visible_hexes.tiles {
        if let Some(visible_tile) = tile_data.entities.get(hex) {
            visible_tiles.entities.push(*visible_tile);
        }
    }
    commands.trigger(super::events::VisibleTilesUpdated);
}

pub fn remove_tiles(
    _trigger: On<input::events::RemoveTile>,
    mut tile_data: ResMut<super::resources::TileData>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    tile_data.entities.remove(&hex);
    commands.trigger(super::events::TileDataUpdated);
}

pub fn set_tile(
    _trigger: On<input::events::SetTile>,
    mut tile_data: ResMut<super::resources::TileData>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let hex = Hex::from_world(mouse_pos.world);
    let tile_entity = commands
        .spawn(super::bundles::HexTile::new(hex, TerrainType::Water))
        .id();

    tile_data.entities.insert(hex, tile_entity);
    commands.trigger(super::events::TileDataUpdated);
}

pub fn from_hex_to_world(hex: Hex) -> Vec2 {
    let q = hex.q as f32;
    let r = hex.r as f32;

    let x = HEX_WIDTH * SQRT_3 * (q + r * 0.5);

    let y = HEX_HEIGHT * THREE_HALVES * r;

    Vec2::new(x.round(), y.round())
}

pub fn from_world_to_hex(pixel: Vec2) -> Hex {
    let px = pixel.x / HEX_WIDTH;
    let py = pixel.y / HEX_HEIGHT;

    let r_float = py * TWO_THIRDS;

    let q_float = (px * SQRT_3_OVER_3) - (r_float * 0.5);

    hex_round(q_float, r_float)
}

fn hex_round(frac_q: f32, frac_r: f32) -> Hex {
    let frac_s = -frac_q - frac_r;

    let mut q = frac_q.round();
    let mut r = frac_r.round();
    let s = frac_s.round();

    let q_diff = (q - frac_q).abs();
    let r_diff = (r - frac_r).abs();
    let s_diff = (s - frac_s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    }

    Hex::new(q as i32, r as i32)
}
