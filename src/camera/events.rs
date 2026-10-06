use bevy::prelude::*;

/// Fired when the visible world rect (`PlayerView`) changes.
#[derive(Event)]
pub struct PlayerViewUpdated;
