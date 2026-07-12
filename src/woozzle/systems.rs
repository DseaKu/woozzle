use crate::input;
use crate::map;
use bevy::prelude::*;

pub fn set_woozle(
    _trigger: On<input::events::SpawnWoozleEvent>,
    mut woozzle_data: ResMut<super::resources::WoozlesData>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let woozzle_entity = commands
        .spawn(super::bundles::Woozzle::new(mouse_pos.world))
        .id();
    let hex = map::components::Hex::from_world(mouse_pos.world);
    woozzle_data.entities.insert(hex, woozzle_entity);
}

pub fn update_visible_woozzles(
    viewport_hexes: Res<map::resources::ViewportHexes>,
    mut visible_woozzles: ResMut<super::resources::VisibleWoozzle>,
    woozzle_data: Res<super::resources::WoozlesData>,
) {
    crate::guard_update!(viewport_hexes.is_changed() || woozzle_data.is_changed());

    for hex in &viewport_hexes.tiles {
        if !woozzle_data.entities.contains_key(hex) {
            continue;
        }
        let woozzle_entity = woozzle_data.entities.get(hex).unwrap();
        visible_woozzles.entities.push(*woozzle_entity);
    }
}
