use bevy::prelude::*;

pub mod components;
pub mod major_jobs;
mod minor_jobs;
mod systems;

#[derive(Component)]
pub struct JobsPlugin;
impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, systems::process_job)
            .add_systems(FixedUpdate, minor_jobs::wait)
            .add_systems(FixedUpdate, minor_jobs::ghost_system)
            .add_systems(FixedUpdate, minor_jobs::go_to_point);
    }
}
