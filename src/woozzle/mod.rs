use bevy::prelude::*;

use crate::woozzle::systems::spawn_woozle;

mod bundle;
mod components;
mod resources;
mod systems;

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::VisibleWoozles>()
            .add_observer(spawn_woozle);
    }
}
