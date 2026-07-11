use super::resources;
use crate::map;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";
const TILESET_PATH: &str = "images/tileset16.png";
const TILE_SIZE: u32 = 16;

pub fn despawn_invisble_tiles(
    visible_tiles: Res<map::resources::VisibleTiles>,
    map_data: Res<map::resources::MapData>,
    mut spawned_tiles: ResMut<resources::SpawnedTileTexutures>,
    mut commands: Commands,
) {
    crate::guard_update!(visible_tiles.is_changed() || map_data.is_changed());
    spawned_tiles.tiles.retain(|hex, entity| {
        if visible_tiles.tiles.contains(hex) && map_data.tiles.contains_key(hex) {
            true // Keep the tile
        } else {
            commands.entity(*entity).despawn();
            false // Remove the tile
        }
    });
}

pub fn spawn_visible_tiles(
    visible_tiles: Res<map::resources::VisibleTiles>,
    map_data: Res<map::resources::MapData>,
    mut spawned_tiles: ResMut<resources::SpawnedTileTexutures>,
    mut commands: Commands,
    tile_assets: Res<resources::TilesetAsset>,
) {
    crate::guard_update!(visible_tiles.is_changed() || map_data.is_changed());

    for hex in &visible_tiles.tiles {
        if let Some(terrain_type) = map_data.tiles.get(hex) {
            // Skip already spawned tiles
            if spawned_tiles.tiles.contains_key(hex) {
                continue;
            }

            let entity = commands
                .spawn((
                    Sprite::from_atlas_image(
                        tile_assets.image.clone(),
                        TextureAtlas {
                            layout: tile_assets.layout.clone(),
                            index: terrain_type.to_atlas_index(),
                        },
                    ),
                    Transform::from_xyz(hex.to_world().x, hex.to_world().y, 0.0),
                ))
                .id();

            spawned_tiles.tiles.insert(*hex, entity);
        }
    }
}

pub fn load_tileset_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_SIZE, TILE_SIZE),
        map::components::TerrainType::n_of_types() as u32,
        1,
        None,
        None,
    );
    commands.insert_resource(resources::TilesetAsset {
        image: asset_server.load::<Image>(TILESET_PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!("Loading tileset, from: {}{}", PATH_PREFIX, TILESET_PATH);
}
