use bevy::prelude::*;

const LEFT_MARGIN: f32 = 20.0;
const TOP_MARGIN: f32 = 20.0;
const INDENTED_MARGIN: f32 = 30.0;

#[derive(Component)]
pub struct RootNodeLabel;

#[derive(Component)]
pub struct MouseWorldPosTextLabel;

#[derive(Bundle)]
pub struct RootNodeBundle {
    node: Node,
    label: RootNodeLabel,
}

impl RootNodeBundle {
    pub fn new() -> Self {
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
pub struct SubNode {
    text: Text,
    node: Node,
}

impl SubNode {
    pub fn new(text: &str) -> Self {
        Self {
            text: Text::new(text),
            node: Node {
                flex_direction: FlexDirection::Column,
                ..default()
            },
        }
    }
    pub fn new_indented(text: &str) -> Self {
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

#[derive(Component)]
pub struct MousePosText;

#[derive(Component)]
pub struct HexPosText;
