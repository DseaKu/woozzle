use bevy::prelude::*;

#[derive(Component)]
pub struct WoozzleSprite {
    sprite: Sprite,
}

impl WoozzleSprite {
    pub fn new(assets: &super::resources::WoozzleAsset) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(
                assets.image.clone(),
                TextureAtlas {
                    layout: assets.layout.clone(),
                    index: 1,
                },
            ),
        }
    }
}
