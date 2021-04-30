use crate::components::{Label, LabelType};
use crate::resources::*;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

// 表示するラベル
pub fn label_update(
    gameplay: Res<Gameplay>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &Label)>,
){
    for (mut text, label) in query.iter_mut(){
        match label.label_type {
            // 移動回数を表示
            LabelType::MovesCount => (*text).sections[0].value = format!("Moves: {}", gameplay.moves_count),
            // ゲームの状態を表示
            LabelType::GameplayState => (*text).sections[0].value = format!("{}", gameplay.state),
            // フレームレートを表示
            LabelType::FPS => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average(){
                        (*text).sections[0].value = format!("FPS: {:.0}", average);
                    }
                }
            }
        }
    }
}