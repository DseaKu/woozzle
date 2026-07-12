use crate::map;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileSprite {
    sprite: Sprite,
    transform: Transform,
}
impl TileSprite {
    pub fn new(hex: map::components::Hex, image: Handle<Image>, atlas: TextureAtlas) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(image, atlas),
            transform: Transform::from_xyz(hex.to_world().x, hex.to_world().y, 0.0),
        }
    }
}
