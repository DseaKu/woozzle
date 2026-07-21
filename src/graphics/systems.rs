use super::components::*;
use super::resources;
use super::resources::*;
use crate::map;
use crate::woozzle;
use bevy::prelude::*;

const PATH_PREFIX: &str = "/assets/";

pub fn insert_woozzle_sprite(
    _trigger: On<woozzle::events::VisibleUpdated>,
    visible_woozzles: Res<woozzle::resources::Visible>,
    existing_sprites: Query<Entity, With<super::components::VisibleLabel>>,
    mut commands: Commands,
    woozzle_asset: Res<WoozzleAsset>,
) {
    use super::components::{SpriteAnimation, VisibleLabel, WoozzleSprite};

    for woozzle in &visible_woozzles.entities {
        if existing_sprites.get(*woozzle).is_err() {
            commands.entity(*woozzle).insert((
                WoozzleSprite::new(&woozzle_asset),
                SpriteAnimation::new(6.0, 0, 1),
                VisibleLabel,
            ));
        }
    }
}
pub fn remove_woozzle_sprite(
    _trigger: On<woozzle::events::VisibleUpdated>,
    visible_woozzles: Res<woozzle::resources::Visible>,
    mut commands: Commands,
    woozzle_query: Query<Entity, With<VisibleLabel>>,
) {
    use super::components::{SpriteAnimation, VisibleLabel, WoozzleSprite};

    for actual_visible_woozzle in woozzle_query {
        if visible_woozzles.entities.contains(&actual_visible_woozzle) {
            continue;
        }
        commands
            .entity(actual_visible_woozzle)
            .remove::<(WoozzleSprite, SpriteAnimation, VisibleLabel)>();
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut Sprite), With<VisibleLabel>>,
) {
    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index < anim.first_frame || atlas.index >= anim.last_frame {
                atlas.index = anim.first_frame;
            } else {
                atlas.index += 1;
            }
        }
    }
}

pub fn remove_tile_sprite(
    _trigger: On<map::events::VisibleUpdated>,
    visible_tiles: Res<map::resources::Visible>,
    mut commands: Commands,
    tile_query: Query<Entity, With<VisibleTileLabel>>,
) {
    use super::components::{TileSprite, VisibleTileLabel};
    for actual_visible_tiles in tile_query {
        if visible_tiles.entities.contains(&actual_visible_tiles) {
            continue;
        }
        commands
            .entity(actual_visible_tiles)
            .remove::<(TileSprite, VisibleTileLabel)>();
    }
}

pub fn insert_tile_sprite(
    _trigger: On<map::events::VisibleUpdated>,
    visible_tiles: Res<map::resources::Visible>,
    mut commands: Commands,
    tile_assets: Res<resources::TilesetAsset>,
) {
    use super::components::{TileSprite, VisibleTileLabel};
    for tile in &visible_tiles.entities {
        commands.entity(*tile).insert((
            TileSprite::new(&tile_assets, map::components::TerrainType::Water),
            VisibleTileLabel,
        ));
    }
}
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
