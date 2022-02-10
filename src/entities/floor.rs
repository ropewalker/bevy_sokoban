use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_floors(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    positions: Vec<Position>,
) {
    for position in positions {
        let transform = position_to_translation(map, tile_size, &position, 5.0);

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("images/floor.png"),
                transform,
                ..Default::default()
            })
            .insert(position);
    }
}
