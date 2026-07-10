use super::resources::TilesetAsset;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";
const TILESET_PATH: &str = "images/tileset.png";

// pub fn spawn_tiles_texture() {}

pub fn load_tileset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 1, None, None);
    commands.insert_resource(TilesetAsset {
        image: asset_server.load::<Image>(TILESET_PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading tileset, from: {}{}", PATH_PREFIX, TILESET_PATH);
}
