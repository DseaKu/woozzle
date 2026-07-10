use bevy::prelude::*;

mod components;
mod resources;
mod system;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system::load_tileset_assets)
            .init_resource::<resources::TilesetAsset>()
            .init_resource::<resources::SpawnedTileTexutures>();
    }
}
