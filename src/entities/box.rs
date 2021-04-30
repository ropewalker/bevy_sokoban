use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

// Res<>:
//  リソースへのRead/Writeを行うためのポインタ

// AssetServer:
//  bevyが提供するバックグラウンドスレッド上でファイルシステムからアセットをロードする

//  マップ上に箱を作る
pub fn create_boxes(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    positions_by_color: HashMap<BoxColor, Vec<Position>>,
){
    for (color, positions) in positions_by_color {
        //指定のテクスチャのハンドルを取得
        let texture_handle =
            asset_server.get_handle(format!("images/box_{}_spritesheet.png", color).as_str());
        //指定のテクスチャをグリッドにしてアトラスを生成
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(tile_size.0, tile_size.0), 2, 1);
        //Resourcesにアトラスを追加
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        for position in positions {
            let transform = position_to_translation(map, tile_size, &position, 10.0);
            // アトラスからテクスチャを生成して、スプライトをワールドに描画する
            commands
                .spawn()
                .insert_bundle(
                    SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform,
                    ..Default::default() // Defaultトレイト、Default以外の変更点だけ書けば良い
                })
                .insert(Timer::from_seconds(0.25, true))
                .insert(position)
                .insert(Box {color})
                .insert(Movable);
        }
    }
}