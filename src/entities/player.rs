use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_players(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    positions: Vec<Position>,
){
    let texture_handle = asset_server.get_handle("images/player_spritesheet.png");
    let texture_atlas =
    TextureAtlas::from_grid(texture_handle, Vec2::new(tile_size.0, tile_size.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for position in positions {
        let transform = position_to_translation(map, tile_size, &position, 10.0);

        commands
            .spawn()
            .insert_bundle(SpriteSheetBundle{
                texture_atlas: texture_atlas_handle.clone(),
                transform,
                ..Default::default()
            })
            .insert(Timer::from_seconds(0.25, true))
            .insert(position)
            .insert(Player)
            .insert(Movable);
    }
}