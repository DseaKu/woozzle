use super::resources::*;
use crate::map;
use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub first_frame: usize,
    pub last_frame: usize,
}

impl SpriteAnimation {
    pub fn new(fps: f32, first_frame: usize, last_frame: usize) -> Self {
        Self {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            first_frame,
            last_frame,
        }
    }
}

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
    pub fn new(assets: &TilesetAsset, terrain_type: map::components::TerrainType) -> Self {
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
pub struct VisibleLabel;

#[derive(Component)]
pub struct VisibleTileLabel;
