use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_walls(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    positions: Vec<Position>,
) {
    for position in positions {
        let transform = position_to_translation(map, tile_size, &position, 10.0);

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("images/wall.png"),
                transform,
                ..Default::default()
            })
            .insert(position)
            .insert(Wall)
            .insert(Immovable);
    }
}
