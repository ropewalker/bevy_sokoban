use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

// 更新された対象を移動させる
pub fn translation(
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut query: Query<(&mut Transform, &Position), Changed<Position>>, // Changed で 更新があったものだけをFilterする
){
    for (mut transform, position) in query.iter_mut(){
        *transform =
            position_to_translation(&map, &tile_size, &(*position), transform.translation.z);
    }
}