use bevy::prelude::*;

mod system;

const LEFT_MARGIN: f32 = 20.0;
const TOP_MARGIN: f32 = 20.0;
const INDENTED_MARGIN: f32 = 30.0;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(system::SystemPlugin);
    }
}

#[derive(Component)]
struct RootNodeLabel;
#[derive(Bundle)]
struct RootNodeBundle {
    node: Node,
    label: RootNodeLabel,
}
impl RootNodeBundle {
    fn new() -> Self {
        Self {
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(LEFT_MARGIN),
                top: Val::Px(TOP_MARGIN),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            label: RootNodeLabel,
        }
    }
}

#[derive(Bundle)]
struct SubNode {
    text: Text,
    node: Node,
}
impl SubNode {
    fn new(text: &str) -> Self {
        Self {
            text: Text::new(text),
            node: Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        }
    }
    fn new_indented(text: &str) -> Self {
        Self {
            text: Text::new(text),
            node: Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::left(Val::Px(INDENTED_MARGIN)),
                ..default()
            },
        }
    }
}
