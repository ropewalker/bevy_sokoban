use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn create_box_spots(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    positions_by_color: HashMap<BoxColour, Vec<Position>>,
) {
    for (colour, positions) in positions_by_color {

        for position in positions {
            let transform = position_to_translation(map, tile_size, &position, 9.0);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(format!("images/box_spot_{}.png", colour).as_str()),
                    transform,
                    ..Default::default()
                })
                .insert(position)
                .insert(BoxSpot { colour });
        }
    }
}
