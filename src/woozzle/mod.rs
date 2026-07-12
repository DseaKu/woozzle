use bevy::prelude::*;

mod bundles;
mod components;
pub mod resources;
mod systems;

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::WoozlesData>()
            .init_resource::<resources::VisibleWoozzle>()
            .add_systems(Update, systems::update_visible_woozzles)
            .add_observer(systems::set_woozle);
    }
}
