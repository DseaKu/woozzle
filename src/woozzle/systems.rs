use crate::{camera, input, map};
use bevy::prelude::*;

pub fn set_woozle(
    _trigger: On<input::events::SpawnWoozle>,
    mut woozzle_data: ResMut<super::resources::WoozlesData>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let woozzle_entity = commands
        .spawn(super::bundles::Woozzle::new(mouse_pos.world))
        .id();

    let hex = map::components::Hex::from_world(mouse_pos.world);

    woozzle_data
        .entities
        .entry(hex)
        .or_default()
        .push(woozzle_entity);

    commands.trigger(super::events::WoozzleDataUpdated);
}

pub fn update_visible_woozzles<E: Event>(
    _trigger: On<E>,
    visible_hexes: Res<camera::resources::VisibleHexes>,
    mut visible_woozzles: ResMut<super::resources::VisibleWoozzle>,
    woozzle_data: Res<super::resources::WoozlesData>,
    mut commands: Commands,
) {
    visible_woozzles.entities.clear();
    for hex in &visible_hexes.tiles {
        if let Some(woozzles_in_hex) = woozzle_data.entities.get(hex) {
            for &woozzle_entity in woozzles_in_hex {
                visible_woozzles.entities.push(woozzle_entity);
            }
        }
    }
    commands.trigger(super::events::VisibleWoozzleUpdated);
}
