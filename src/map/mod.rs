use bevy::prelude::*;

pub mod components;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::set_tile);
    }
}
