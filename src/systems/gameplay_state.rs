use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

// ゲームの状態管理(ゲーム中 or ゲーム終了)
pub fn gameplay_state(
    mut gameplay: ResMut<Gameplay>,
    box_spot_query: Query<(&BoxSpot, &Position)>,
    box_query: Query<(&Box, &Position)>,
){
    // BoxSpotの位置ごとの色を連想配列で持つ
    let occupied_positions_with_colors = box_query
        .iter() // イテレータとして扱う
        .map(|t| (*t.1, t.0.color)) // 各要素を取り出して ( position, color(色)に置き換える
        .collect::<HashMap<_, _>>(); // HashMap型として中身を置き換える、positionをキーとして色を取得する連想配列となる

    for (box_spot, position) in &mut box_spot_query.iter(){
        if let Some(&color) = occupied_positions_with_colors.get(position) {
            // 色がスポット用のものに変わってないスポットが1つでもあればゲ-ムを継続
            if color != box_spot.color {
                gameplay.state = GameplayState::Playing;
                return;
            }
        }else{
            // 色がないスポットが1つでもあればゲ-ムを継続
            gameplay.state = GameplayState::Playing;
            return;
        }
    }
    gameplay.state = GameplayState::Won;
}