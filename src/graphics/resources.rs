use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct WoozzleAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
