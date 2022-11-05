use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation(
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, position) in query.iter_mut() {
        *transform = position_to_translation(&map, &tile_size, position, transform.translation.z);
    }
}
