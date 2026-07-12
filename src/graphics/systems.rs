use super::resources;
use crate::map;
use crate::woozzle;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";
const TILESET_PATH: &str = "images/tileset16.png";
const WOOZZLE_PATH: &str = "images/woozzle.png";
const TILE_SIZE: u32 = 16;

pub fn insert_woozzle_sprite(
    visible_woozzles: Res<woozzle::resources::VisibleWoozzle>,
    mut commands: Commands,
    woozzle_asset: Res<super::resources::WoozzleAsset>,
) {
    crate::guard_update!(visible_woozzles.is_changed());

    for woozzle in &visible_woozzles.entities {
        commands
            .entity(*woozzle)
            .insert(super::components::WoozzleSprite::new(&woozzle_asset));
    }
}

pub fn despawn_tiles(
    viewport_hexes: Res<map::resources::ViewportHexes>,
    map_data: Res<map::resources::MapData>,
    mut tile_sprite_entities: ResMut<resources::TileSpriteEntities>,
    mut commands: Commands,
) {
    crate::guard_update!(viewport_hexes.is_changed() || map_data.is_changed());

    tile_sprite_entities.entities.retain(|hex, entity| {
        if viewport_hexes.tiles.contains(hex) && map_data.tiles.contains_key(hex) {
            true // Keep the tile
        } else {
            commands.entity(*entity).despawn();
            false // Remove the tile
        }
    });
}

pub fn spawn_tiles(
    viewport_hexes: Res<map::resources::ViewportHexes>,
    map_data: Res<map::resources::MapData>,
    mut tile_sprite_entities: ResMut<resources::TileSpriteEntities>,
    mut commands: Commands,
    tile_assets: Res<resources::TilesetAsset>,
) {
    crate::guard_update!(viewport_hexes.is_changed() || map_data.is_changed());

    for hex in &viewport_hexes.tiles {
        if let Some(terrain_type) = map_data.tiles.get(hex) {
            // Skip already spawned tiles
            if tile_sprite_entities.entities.contains_key(hex) {
                continue;
            }
            let tile_entity = commands
                .spawn(map::bundles::TileSprite::new(
                    *hex,
                    &tile_assets,
                    *terrain_type,
                ))
                .id();

            tile_sprite_entities.entities.insert(*hex, tile_entity);
        }
    }
}
pub fn load_woozzle_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_SIZE, TILE_SIZE), 1, 1, None, None);
    commands.insert_resource(resources::WoozzleAsset {
        image: asset_server.load::<Image>(WOOZZLE_PATH),
        layout: texture_atlas_layouts.add(layout),
    });
    println!(
        "Loading Woozzle asset, from: {}{}",
        PATH_PREFIX, WOOZZLE_PATH
    );
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
