use super::events::*;
use super::resources::*;
use crate::input::events::ToggleJobMode;
use crate::jobs::components::{ActionQueue, GoToPoint, Idle};
use crate::jobs::planners::plan_rectangle_patrol;
use crate::jobs::planners::plan_wandering;
use crate::woozzle;
use crate::woozzle::components;
use crate::woozzle::resources;
use crate::{input, map};
use avian2d::prelude::*;
use bevy::prelude::*;
use std::cmp::Ordering;

pub fn rebuild_woozzles_by_hex(
    mut woozzles_by_hex: ResMut<woozzle::resources::WoozzlesByHex>,
    query: Query<(Entity, &Transform), With<woozzle::components::Woozzle>>,
    mut commands: Commands,
) {
    // Clear the old  map
    woozzles_by_hex.entities.clear();

    // Repopulate the map with each Woozzle's current hex location
    for (entity, transform) in &query {
        let current_hex = map::components::Hex::from_world(transform.translation.truncate());

        woozzles_by_hex
            .entities
            .entry(current_hex)
            .or_default()
            .push(entity);
    }
    commands.trigger(WoozzlesByHexUpdated);
}
pub fn toggle_job_mode(_trigger: On<ToggleJobMode>, mut job_mode: ResMut<resources::JobMode>) {
    *job_mode = match *job_mode {
        JobMode::Wander => JobMode::Patrol,
        JobMode::Patrol => JobMode::Wander,
    };
}

pub fn update_facing(
    mut query: Query<(Entity, &LinearVelocity, &mut Sprite), With<components::NeedsFacingUpdate>>,
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
            .remove::<components::NeedsFacingUpdate>();
    }
}

type WoozzleSpriteQuery<'a> = (
    &'a mut crate::graphics::components::SpriteAnimation,
    Option<&'a GoToPoint>,
);

pub fn update_walk_animation(mut query: Query<WoozzleSpriteQuery, With<components::Woozzle>>) {
    for (mut anim, go_to_point) in &mut query {
        if go_to_point.is_some() {
            if anim.first_frame != 2 {
                anim.first_frame = 2;
                anim.last_frame = 3;
            }
        } else if anim.first_frame != 0 {
            anim.first_frame = 0;
            anim.last_frame = 1;
        }
    }
}

pub fn request_facing_updates(
    query: Query<Entity, With<components::Woozzle>>,
    mut commands: Commands,
) {
    for woozzle in query {
        commands
            .entity(woozzle)
            .insert(components::NeedsFacingUpdate);
    }
}

pub fn assign_job(
    job_mode: Res<JobMode>,
    query: Query<(Entity, &mut ActionQueue), With<Idle>>,
    mut commands: Commands,
) {
    for (woozzle, mut empty_queue) in query {
        match *job_mode {
            JobMode::Patrol => plan_rectangle_patrol(&mut empty_queue, Vec2::ZERO, 500.0),
            JobMode::Wander => plan_wandering(&mut empty_queue, Vec2::ZERO, 1200.0),
        }

        commands.entity(woozzle).remove::<Idle>();
    }
}

pub fn spawn_woozzle(
    _trigger: On<input::events::SpawnWoozzle>,
    mut woozzles_by_hex: ResMut<WoozzlesByHex>,
    mouse_pos: Res<input::resources::MousePos>,
    mut commands: Commands,
) {
    let new_woozzle = commands
        .spawn(super::bundles::WoozzleBundle::new(mouse_pos.world))
        .id();

    let hex = map::components::Hex::from_world(mouse_pos.world);

    woozzles_by_hex
        .entities
        .entry(hex)
        .or_default()
        .push(new_woozzle);

    commands.trigger(WoozzlesByHexUpdated);
}
