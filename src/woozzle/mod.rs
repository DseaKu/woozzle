use crate::camera;
use bevy::prelude::*;

mod bundles;
mod components;
pub mod events;
pub mod resources;
mod systems;

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::WoozlesData>()
            .init_resource::<resources::VisibleWoozzle>()
            .add_observer(systems::update_visible_woozzles::<events::WoozzleDataUpdated>)
            .add_observer(systems::update_visible_woozzles::<camera::events::VisibleHexesUpdated>)
            .add_observer(systems::set_woozle);
    }
}
