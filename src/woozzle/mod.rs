use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

pub mod bundles;
pub mod components;
pub mod events;
pub mod resources;
mod systems;

use std::time::Duration;

const FACING_UPDATE_INTERVAL: f32 = 0.3;
const WOOZZLES_BY_HEX_INTERVAL: f32 = 0.3; // The function runs through every woozzle

pub struct WoozzlePlugin;
impl Plugin for WoozzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::WoozzlesByHex>()
            .add_systems(Update, systems::assign_job)
            .add_systems(Update, systems::update_facing)
            .add_systems(Update, systems::update_walk_animation)
            .add_systems(
                Update,
                systems::request_facing_updates
                    .run_if(on_timer(Duration::from_secs_f32(FACING_UPDATE_INTERVAL))),
            )
            .add_systems(
                Update,
                systems::rebuild_woozzles_by_hex
                    .run_if(on_timer(Duration::from_secs_f32(WOOZZLES_BY_HEX_INTERVAL))),
            )
            .init_resource::<resources::JobMode>()
            .add_observer(systems::toggle_job_mode)
            .add_observer(systems::spawn_woozzle);
    }
}
