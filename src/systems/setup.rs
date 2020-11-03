use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn setup(
    mut commands: Commands,
    map: Res<Map>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default());

    create_labels(&mut commands, &asset_server);

    let mut wall_positions = Vec::new();
    let mut floor_positions = Vec::new();
    let mut player_positions = Vec::new();
    let mut box_positions_by_colour: HashMap<BoxColour, Vec<Position>> = HashMap::new();
    let mut box_spot_positions_by_colour: HashMap<BoxColour, Vec<Position>> = HashMap::new();

    for (x, y) in map.tiles.keys() {
        let position = Position { x: *x, y: *y };

        if let Some(c) = map.tiles.get(&(*x, *y)) {
            match &c[..] {
                "." => floor_positions.push(position),
                "W" => {
                    floor_positions.push(position);
                    wall_positions.push(position);
                }
                "P" => {
                    floor_positions.push(position);
                    player_positions.push(position);
                }
                "BB" => {
                    floor_positions.push(position);
                    box_positions_by_colour
                        .entry(BoxColour::Blue)
                        .or_default()
                        .push(position);
                }
                "RB" => {
                    floor_positions.push(position);
                    box_positions_by_colour
                        .entry(BoxColour::Red)
                        .or_default()
                        .push(position);
                }
                "BS" => {
                    floor_positions.push(position);
                    box_spot_positions_by_colour
                        .entry(BoxColour::Blue)
                        .or_default()
                        .push(position);
                }
                "RS" => {
                    floor_positions.push(position);
                    box_spot_positions_by_colour
                        .entry(BoxColour::Red)
                        .or_default()
                        .push(position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }

    create_floors(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        floor_positions,
    );
    create_walls(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        wall_positions,
    );
    create_boxes(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        box_positions_by_colour,
    );
    create_box_spots(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        box_spot_positions_by_colour,
    );
    create_players(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        player_positions,
    );
}
