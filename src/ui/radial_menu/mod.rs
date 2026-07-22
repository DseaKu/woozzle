use bevy::prelude::*;

pub mod events;
mod systems;

pub struct RadialMenuPlugin;
impl Plugin for RadialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(systems::open_radial_menu);
    }
}
