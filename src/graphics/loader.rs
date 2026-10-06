use super::resources;
use crate::map;
use bevy::prelude::*;

// Only used for log output
const PATH_PREFIX: &str = "/assets/";

/// Woozzle sheet: 2x2 grid of 16px frames (0-1 idle, 2-3 walk).
pub fn load_woozzle_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const PATH: &str = "images/woozzle.png";
    const SIZE: u32 = 16;
    const COLUMNS: u32 = 2;
    const ROWS: u32 = 2;

    let layout = TextureAtlasLayout::from_grid(UVec2::new(SIZE, SIZE), COLUMNS, ROWS, None, None);
    commands.insert_resource(resources::WoozzleAtlas {
        image: asset_server.load::<Image>(PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading Woozzle asset, from: {}{}", PATH_PREFIX, PATH);
}

/// Tileset: 32px tiles, one column per `TerrainType`.
pub fn load_tileset_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const PATH: &str = "images/tileset32.png";
    const SIZE: u32 = 32;
    const ROWS: u32 = 2;

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SIZE, SIZE),
        map::components::TerrainType::count() as u32,
        ROWS,
        None,
        None,
    );
    commands.insert_resource(resources::TilesetAtlas {
        image: asset_server.load::<Image>(PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading tileset, from: {}{}", PATH_PREFIX, PATH);
}
