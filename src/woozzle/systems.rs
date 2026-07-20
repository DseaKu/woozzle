use super::events::*;
use super::resources::*;
use crate::jobs::components::{ActionQueue, JobLess};
use crate::jobs::major_jobs::assign_rectangle_patrol;
use crate::jobs::major_jobs::wandering;
use crate::woozzle::components;
use crate::{camera, input, map};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn update_sprite_facing(
    mut query: Query<(&LinearVelocity, &mut Sprite), With<components::Woozzle>>,
) {
    for (velocity, mut sprite) in &mut query {
        if velocity.0.x < -0.1 {
            sprite.flip_x = false;
        } else if velocity.0.x > 0.1 {
            sprite.flip_x = true;
        }
    }
}

pub fn get_a_job(query: Query<(Entity, &mut ActionQueue), With<JobLess>>, mut commands: Commands) {
    for (woozzle, mut empty_queue) in query {
        if false {
            assign_rectangle_patrol(&mut empty_queue, Vec2 { x: 30.0, y: 10.0 }, 70.0);
        } else {
            wandering(&mut empty_queue, Vec2 { x: 0.0, y: 0.0 }, 200.0);
        }

        commands.entity(woozzle).remove::<JobLess>();
    }
}

pub fn set_woozle(
    _trigger: On<input::events::SpawnWoozle>,
    mut woozzle_data: ResMut<Data>,
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

    commands.trigger(DataUpdated);
}

pub fn update_visible_woozzles<E: Event>(
    _trigger: On<E>,
    visible_hexes: Res<camera::resources::VisibleHexes>,
    mut visible_woozzles: ResMut<Visible>,
    woozzle_data: Res<Data>,
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
    commands.trigger(VisibleUpdated);
}
