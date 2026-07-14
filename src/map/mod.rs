use bevy::prelude::*;

use crate::camera;

pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::set_tile)
            .add_observer(systems::remove_tiles)
            .add_observer(systems::update_visible_tiles::<camera::events::VisibleHexesUpdated>)
            .add_observer(systems::update_visible_tiles::<events::DataUpdated>)
            .init_resource::<resources::Data>()
            .init_resource::<resources::Visible>();
    }
}
