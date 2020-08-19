use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut translation: Mut<Translation>,
    position: &Position,
) {
    *translation = position_to_translation(&map, &tile_size, position, translation.0.z());
}
