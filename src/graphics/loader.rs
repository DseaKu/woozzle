use super::resources;
use crate::map;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";

pub fn load_woozzle_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const PATH: &str = "images/woozzle.png";
    const SIZE: u32 = 16;
    const COLUMNS: u32 = 2;
    const ROWS: u32 = 2;

    let layout = TextureAtlasLayout::from_grid(UVec2::new(SIZE, SIZE), COLUMNS, ROWS, None, None);
    commands.insert_resource(resources::WoozzleAsset {
        image: asset_server.load::<Image>(PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading Woozzle asset, from: {}{}", PATH_PREFIX, PATH);
}

pub fn load_tileset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const PATH: &str = "images/tileset32.png";
    const SIZE: u32 = 32;
    const ROWS: u32 = 2;

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SIZE, SIZE),
        map::components::TerrainType::n_of_types() as u32,
        ROWS,
        None,
        None,
    );
    commands.insert_resource(resources::TilesetAsset {
        image: asset_server.load::<Image>(PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading tileset, from: {}{}", PATH_PREFIX, PATH);
}
