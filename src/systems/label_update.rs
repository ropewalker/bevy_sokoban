use crate::components::{Label, LabelType};
use crate::resources::*;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub fn label_update(
    gameplay: Res<Gameplay>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &Label)>,
) {
    for (mut text, label) in query.iter_mut() {
        match label.label_type {
            LabelType::MovesCount => (*text).value = format!("Moves: {}", gameplay.moves_count),
            LabelType::GameplayState => (*text).value = format!("{}", gameplay.state),
            LabelType::FPS => {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        (*text).value = format!("FPS: {:.0}", average);
                    }
                }
            }
        }
    }
}
