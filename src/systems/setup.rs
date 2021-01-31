use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup(
    commands: &mut Commands,
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(Camera2dBundle::default());

    asset_server.load_folder("images").unwrap();
    asset_server.load_folder("fonts").unwrap();
    sound_handles.handles = asset_server.load_folder("sounds").unwrap();

    create_labels(commands, &asset_server);

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
        commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        floor_positions,
    );
    create_walls(
        commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        wall_positions,
    );
    create_boxes(
        commands,
        &map,
        &tile_size,
        &asset_server,
        &mut texture_atlases,
        box_positions_by_colour,
    );
    create_box_spots(
        commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        box_spot_positions_by_colour,
    );
    create_players(
        commands,
        &map,
        &tile_size,
        &asset_server,
        &mut texture_atlases,
        player_positions,
    );
}
