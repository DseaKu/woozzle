use super::resources;

use crate::map;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";
const TILESET_PATH: &str = "images/tileset.png";

pub fn spawn_visible_tiles(
    visible_tiles: Res<map::resources::VisibleTiles>,
    map_data: Res<map::resources::MapData>,
    mut spawned_tiles: ResMut<resources::SpawnedTileTexutures>,
    mut commands: Commands,
    tile_assets: Res<resources::TilesetAsset>,
) {
    for hex in &visible_tiles.tiles {
        if let Some(terrain_type) = map_data.tiles.get(hex) {
            // Skip already spawned tiles
            if spawned_tiles.tiles.contains_key(hex) {
                continue;
            }

            let entity_id = commands
                .spawn((
                    Sprite::from_atlas_image(
                        tile_assets.image.clone(),
                        TextureAtlas {
                            layout: tile_assets.layout.clone(),
                            index: terrain_type.to_atlas_index(),
                        },
                    ),
                    Transform::from_xyz(hex.to_world().x, hex.to_world().y, 0.0),
                    *hex,
                ))
                .id();
            
            spawned_tiles.tiles.insert(*hex, *terrain_type);
        }
    }
}

pub fn load_tileset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        1,
        map::components::TerrainType::n_of_types() as u32,
        None,
        None,
    );
    commands.insert_resource(resources::TilesetAsset {
        image: asset_server.load::<Image>(TILESET_PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading tileset, from: {}{}", PATH_PREFIX, TILESET_PATH);
}
