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
    let material = materials.add(asset_server.get_handle("images/wall.png").into());

    for position in positions {
        let transform = position_to_translation(map, &tile_size, &position, 10.0);

        commands
            .spawn(SpriteBundle {
                material: material.clone(),
                transform,
                ..Default::default()
            })
            .with(position)
            .with(Wall)
            .with(Immovable);
    }
}
