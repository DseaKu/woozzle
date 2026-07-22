use bevy::prelude::*;

pub mod debug;
pub mod radial_menu;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug::DebugUiPlugin)
            .add_plugins(radial_menu::RadialMenuPlugin);
    }
}
