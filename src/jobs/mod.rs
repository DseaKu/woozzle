use bevy::prelude::*;

pub mod components;
pub mod major_jobs;
mod minor_jobs;
mod systems;

#[derive(Component)]
pub struct JobsPlugin;
impl Plugin for JobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::process_job)
            .add_systems(Update, minor_jobs::go_to_point);
    }
}
