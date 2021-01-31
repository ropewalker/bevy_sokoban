use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn create_boxes(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    positions_by_color: HashMap<BoxColour, Vec<Position>>,
) {
    for (colour, positions) in positions_by_color {
        let texture_handle =
            asset_server.get_handle(format!("images/box_{}_spritesheet.png", colour).as_str());
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(tile_size.0, tile_size.0), 2, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        for position in positions {
            let transform = position_to_translation(map, tile_size, &position, 10.0);

            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform,
                    ..Default::default()
                })
                .with(Timer::from_seconds(0.25, true))
                .with(position)
                .with(Box { colour })
                .with(Movable);
        }
    }
}
