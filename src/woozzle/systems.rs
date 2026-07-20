use super::events::*;
use super::resources::*;
use crate::input::events::ChangeMajorJob;
use crate::jobs::components::{ActionQueue, JobLess};
use crate::jobs::major_jobs::assign_rectangle_patrol;
use crate::jobs::major_jobs::wandering;
use crate::woozzle;
use crate::woozzle::components;
use crate::woozzle::resources;
use crate::{camera, input, map};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::cmp::Ordering;

pub fn update_woozzle_hex_data(
    mut woozzle_data: ResMut<woozzle::resources::Data>,
    query: Query<(Entity, &Transform), With<woozzle::components::Woozzle>>,
    mut commands: Commands,
) {
    // Clear the old  map
    woozzle_data.entities.clear();

    // Repopulate the map with each Woozzle's current hex location
    for (entity, transform) in &query {
        let current_hex = map::components::Hex::from_world(transform.translation.truncate());

        woozzle_data
            .entities
            .entry(current_hex)
            .or_default()
            .push(entity);
    }
    commands.trigger(DataUpdated);
}
pub fn change_major_job(_trigger: On<ChangeMajorJob>, mut flag: ResMut<resources::MajorJobFlag>) {
    match flag.0 {
        true => flag.0 = false,
        false => flag.0 = true,
    }
}

pub fn update_sprite_facing(
    mut query: Query<(Entity, &LinearVelocity, &mut Sprite), With<components::DirtyFaceDir>>,
    mut commands: Commands,
) {
    for (woozzle, velocity, mut sprite) in &mut query {
        // Flip sprite
        match velocity.0.x.partial_cmp(&0.0) {
            Some(Ordering::Less) => sprite.flip_x = false,
            Some(Ordering::Greater) => sprite.flip_x = true,
            _ => {}
        }

        // Remove mark
        commands
            .entity(woozzle)
            .remove::<components::DirtyFaceDir>();
    }
}

pub fn mark_all_face_dir_dirty(
    query: Query<Entity, With<components::Woozzle>>,
    mut commands: Commands,
) {
    for woozzle in query {
        commands.entity(woozzle).insert(components::DirtyFaceDir);
    }
}

pub fn get_a_job(
    job_flag: Res<MajorJobFlag>,
    query: Query<(Entity, &mut ActionQueue), With<JobLess>>,
    mut commands: Commands,
) {
    for (woozzle, mut empty_queue) in query {
        if job_flag.0 {
            assign_rectangle_patrol(&mut empty_queue, Vec2 { x: 30.0, y: 10.0 }, 200.0);
        } else {
            wandering(&mut empty_queue, Vec2 { x: 0.0, y: 0.0 }, 800.0);
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
