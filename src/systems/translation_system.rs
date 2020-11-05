use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut transform: Mut<Transform>,
    position: Changed<Position>,
) {
    *transform = position_to_translation(&map, &tile_size, &(*position), transform.translation.z());
}
