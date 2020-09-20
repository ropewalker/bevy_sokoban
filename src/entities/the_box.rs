use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;
use std::collections::HashMap;

pub fn create_boxes(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    positions_by_color: HashMap<BoxColour, Vec<Position>>,
) {
    for (colour, positions) in positions_by_color {
        let mut texture_atlas_builder = TextureAtlasBuilder::new(
            Vec2::new(tile_size.0 * 2.0, tile_size.0),
            Vec2::new(tile_size.0 * 2.0, tile_size.0),
        );

        for path in vec![
            format!("assets/images/box_{}_1.png", colour),
            format!("assets/images/box_{}_2.png", colour),
        ] {
            let handle: Handle<Texture> = asset_server.load_sync(textures, path).unwrap();
            let texture = textures.get(&handle).unwrap();

            texture_atlas_builder.add_texture(handle, &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(textures).unwrap();
        let texture_atlas = texture_atlases.add(texture_atlas);

        for position in positions {
            let transform = position_to_translation(map, tile_size, &position, 10.0);

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
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
