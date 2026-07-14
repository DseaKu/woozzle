use crate::map;
use bevy::prelude::*;
use super::resources::*;

#[derive(Bundle)]
pub struct WoozzleSprite {
    sprite: Sprite,
}

impl WoozzleSprite {
    pub fn new(assets: &WoozzleAsset) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(
                assets.image.clone(),
                TextureAtlas {
                    layout: assets.layout.clone(),
                    index: 0,
                },
            ),
        }
    }
}

#[derive(Bundle)]
pub struct TileSprite {
    sprite: Sprite,
}
impl TileSprite {
    pub fn new(
        assets: &TilesetAsset,
        terrain_type: map::components::TerrainType,
    ) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(
                assets.image.clone(),
                TextureAtlas {
                    layout: assets.layout.clone(),
                    index: terrain_type.to_atlas_index(),
                },
            ),
        }
    }
}

#[derive(Component)]
pub struct VisibleWoozzleLabel;

#[derive(Component)]
pub struct VisibleTileLabel;
