use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

// マップ上に箱スポットを作る
pub fn create_box_spots(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions_by_color: HashMap<BoxColor, Vec<Position>>,
){
    for (color, positions) in positions_by_color {
        // 画像をマテリアルに追加
        let material = materials.add(
            asset_server.get_handle(format!("images/box_spot_{}.png", color).as_str())
            .into(),
        );

        for position in positions {
            let transform = position_to_translation(map, tile_size, &position, 9.0);
            // ワールドにマテリアルを適用したスプライトを追加する
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    material: material.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(position)
                .insert(BoxSpot {color});
        }
    }
}
