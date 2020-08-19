use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_walls(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions: Vec<Position>,
) {
    let material = materials.add(asset_server.load("assets/images/wall.png").unwrap().into());

    for position in positions {
        let translation = position_to_translation(map, &tile_size, &position, 10.0);

        commands
            .spawn(SpriteComponents {
                material,
                translation,
                ..Default::default()
            })
            .with(position)
            .with(Wall)
            .with(Immovable);
    }
}