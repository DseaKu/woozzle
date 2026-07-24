use bevy::prelude::*;

mod components;
pub mod events;
mod resources;
mod systems;

pub struct RadialMenuPlugin;
impl Plugin for RadialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::RootRadialMenu>()
            .add_observer(systems::open_radial_menu);
    }
}

#[derive(Clone)]
pub enum RadialAction {
    AssignRectanglePatrol,
    AssignWandering,
    SpawnSingleWoozzle,
    SpawnManyWoozzle,
    ToggleDebugUi,
    SetTile,
    SelectTile,
    RemoveTile,
}

#[derive(Clone, Default)]
pub enum RadialNode {
    #[default]
    Empty,
    /// A sub-menu that opens another ring of choices when clicked
    SubMenu {
        label: String,
        color: Color,
        items: Vec<RadialNode>,
    },
    /// A leaf option that triggers a gameplay command
    Action {
        label: String,
        color: Color,
        action: RadialAction, // The gameplay action enum
    },
}
