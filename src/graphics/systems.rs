use super::components::*;
use super::resources::*;
use crate::map;
use crate::woozzle;
use bevy::prelude::*;

// Bevy culls sprites outside the camera by itself, so every entity
// gets its sprite once on spawn.

pub fn insert_woozzle_sprite(
    add: On<Add, woozzle::components::Woozzle>,
    mut commands: Commands,
    woozzle_atlas: Res<WoozzleAtlas>,
) {
    commands.entity(add.entity).insert((
        WoozzleSprite::new(&woozzle_atlas),
        SpriteAnimation::new(6.0, 0, 1),
    ));
}

pub fn insert_tile_sprite(
    add: On<Add, map::components::TerrainType>,
    tiles: Query<&map::components::TerrainType>,
    mut commands: Commands,
    tileset_atlas: Res<TilesetAtlas>,
) {
    let Ok(terrain_type) = tiles.get(add.entity) else {
        return;
    };
    commands
        .entity(add.entity)
        .insert(TileSprite::new(&tileset_atlas, *terrain_type));
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut Sprite, &ViewVisibility)>,
) {
    for (mut anim, mut sprite, view_visibility) in &mut query {
        // Off screen sprites don't need to animate
        if !view_visibility.get() {
            continue;
        }
        anim.timer.tick(time.delta());
        if anim.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index < anim.first_frame || atlas.index >= anim.last_frame {
                atlas.index = anim.first_frame;
            } else {
                atlas.index += 1;
            }
        }
    }
}
