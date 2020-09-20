use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;

pub fn create_players(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    positions: Vec<Position>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::new(
        Vec2::new(tile_size.0 * 3.0, tile_size.0),
        Vec2::new(tile_size.0 * 3.0, tile_size.0),
    );

    for path in vec![
        "assets/images/player_1.png".to_string(),
        "assets/images/player_2.png".to_string(),
        "assets/images/player_3.png".to_string(),
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
            .with(Player)
            .with(Movable);
    }
}
