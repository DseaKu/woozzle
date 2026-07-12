use bevy::{platform::collections::HashMap, prelude::*};

use crate::map;

#[derive(Resource, Default)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// A mapping of Hex coordinates to the actual Bevy Sprite Entities currently spawned in the world.
/// Used by the graphics system to track which tiles are actively rendered on the screen.
#[derive(Resource, Default)]
pub struct TileSpriteEntities {
    pub entities: HashMap<map::components::Hex, Entity>,
}
