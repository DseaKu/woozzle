use crate::ui::radial_menu::RadialNode;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RootRadialMenu {
    node: RadialNode,
}

impl Default for RootRadialMenu {
    fn default() -> Self {
        Self {
            node: RadialNode::SubMenu {
                label: "Main Menu".to_string(),
                color: Color::srgb(0.2, 0.2, 0.2),
                items: vec![],
            },
        }
    }
}
