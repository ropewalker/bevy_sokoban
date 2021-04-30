use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup(
    mut commands: Commands,
    map: Res<Map>,
    tile_size: Res<TileSize>,
    mut sound_handles: ResMut<SoundHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    // カメラのセットアップ
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
    commands.spawn_bundle(camera);
    commands.spawn_bundle(UiCameraBundle::default());

    // アセットをロードする
    asset_server.load_folder("images").unwrap();
    asset_server.load_folder("fonts").unwrap();
    sound_handles.handles = asset_server.load_folder("sounds").unwrap();

    // ラベルを生成する
    create_labels(&mut commands, &asset_server);

    //TODO: 起動時にマップデータを読み込んで、Map上にデータ構造として保持させる
    let mut wall_positions = Vec::new();
    let mut floor_positions = Vec::new();
    let mut player_positions = Vec::new();
    let mut box_positions_by_color: HashMap<BoxColor, Vec<Position>> = HashMap::new();
    let mut box_spot_positions_by_color: HashMap<BoxColor, Vec<Position>> = HashMap::new();

    // マップデータを読み込んで設定
    for (x, y) in map.tiles.keys() {
        let position = Position{x: *x, y: *y};

        if let Some(c) = map.tiles.get(&(*x, *y)){
            match &c[..] {
                // 通路
                "." => floor_positions.push(position),
                // 壁
                "W" => {
                    floor_positions.push(position);
                    wall_positions.push(position);
                }
                // プレイヤー位置
                "P" => {
                    floor_positions.push(position);
                    player_positions.push(position);
                }
                // 青い箱の位置
                "BB" => {
                    floor_positions.push(position);
                    box_positions_by_color
                        .entry(BoxColor::Blue)
                        .or_default()
                        .push(position);
                }
                // 赤い箱の位置
                "RB" => {
                    floor_positions.push(position);
                    box_positions_by_color
                        .entry(BoxColor::Red)
                        .or_default()
                        .push(position);
                }
                // 青い箱のスポット(目的地)の位置
                "BS" => {
                    floor_positions.push(position);
                    box_spot_positions_by_color
                        .entry(BoxColor::Blue)
                        .or_default()
                        .push(position);
                }
                // 赤い箱のスポット(目的地)の位置
                "RS" => {
                    floor_positions.push(position);
                    box_spot_positions_by_color
                        .entry(BoxColor::Red)
                        .or_default()
                        .push(position);
                }
                // 何もないところを示す
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
        box_positions_by_color,
    );
    create_box_spots(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut materials,
        box_spot_positions_by_color,
    );
    create_players(
        &mut commands,
        &map,
        &tile_size,
        &asset_server,
        &mut texture_atlases,
        player_positions,
    );
}