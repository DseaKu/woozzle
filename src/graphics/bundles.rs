use crate::map;
use bevy::prelude::*;

use super::resources;

#[derive(Bundle)]
pub struct TileSprite {
    sprite: Sprite,
    transform: Transform,
}
impl TileSprite {
    pub fn new(hex: map::components::Hex, assets: &resources::TilesetAsset, terrain_type: map::components::TerrainType) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(
                assets.image.clone(),
                TextureAtlas {
                    layout: assets.layout.clone(),
                    index: terrain_type.to_atlas_index(),
                },
            ),
            transform: Transform::from_xyz(hex.to_world().x, hex.to_world().y, 0.0),
        }
    }
}
