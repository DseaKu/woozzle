use super::bundle;
use crate::input;
use bevy::prelude::*;

pub fn spawn_woozle(_trigger: On<input::events::SpawnWoozleEvent>, mut commands: Commands) {
    commands.spawn(bundle::Woozzle::new());
}
