use crate::components::*;
use bevy::prelude::*;

pub fn create_labels(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font_handle = asset_server.get_handle("fonts/FiraSans-Bold.ttf");
    let font_size = 20.0;

    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                font: font_handle.clone(),
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                    alignment: Default::default(),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::GameplayState,
        })
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                font: font_handle.clone(),
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                    alignment: Default::default(),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::MovesCount,
        })
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                font: font_handle,
                style: TextStyle {
                    font_size,
                    color: Color::BLACK,
                    alignment: Default::default(),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Label {
            label_type: LabelType::Fps,
        });
}
