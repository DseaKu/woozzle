use crate::camera;
use bevy::prelude::*;

pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::Data>()
            .add_systems(Update, systems::get_a_job)
            .init_resource::<resources::Visible>()
            .add_observer(systems::update_visible_woozzles::<events::DataUpdated>)
            .add_observer(systems::update_visible_woozzles::<camera::events::VisibleHexesUpdated>)
            .add_observer(systems::set_woozle);
    }
}
