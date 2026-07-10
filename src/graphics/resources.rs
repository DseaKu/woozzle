use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
