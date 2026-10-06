use bevy::prelude::*;

mod components;
pub mod events;
mod resources;
mod systems;

/// Radial context menu (work in progress).
pub struct RadialMenuPlugin;
impl Plugin for RadialMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::RootRadialMenu>()
            .add_observer(systems::open_radial_menu);
    }
}

/// Gameplay commands a menu leaf can trigger.
#[allow(dead_code)]
#[derive(Clone)]
pub enum RadialAction {
    AssignRectanglePatrol,
    AssignWandering,
    SpawnSingleWoozzle,
    SpawnManyWoozzle,
    ToggleDebugUi,
    PlaceTile,
    SelectTile,
    RemoveTile,
}

#[allow(dead_code)]
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
