use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod events;
pub mod hex;
pub mod resources;
pub mod save;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::place_tile)
            .add_plugins(hex::HexPlugin)
            .add_plugins(save::SavePlugin)
            .add_observer(systems::remove_tile)
            .init_resource::<resources::TilesByHex>();
    }
}
