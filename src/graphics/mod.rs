use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::load_tileset_assets)
            .add_systems(Update, systems::despawn_invisble_tiles)
            .add_systems(Update, systems::spawn_visible_tiles)
            .init_resource::<resources::TilesetAsset>()
            .init_resource::<resources::SpawnedTileTexutures>();
    }
}
