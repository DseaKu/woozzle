use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TilesetAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct WoozzleAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}
