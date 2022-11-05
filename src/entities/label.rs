use crate::components::*;
use bevy::prelude::*;
use bevy::ui::UiRect;

pub fn create_labels(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font_handle = asset_server.get_handle("fonts/FiraSans-Bold.ttf");
    let font_size = 20.0;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: font_handle.clone(),
                    font_size,
                    color: Color::BLACK,
                },
            ),
            ..Default::default()
        })
        .insert(Label {
            label_type: LabelType::GameplayState,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(font_size),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: font_handle.clone(),
                    font_size,
                    color: Color::BLACK,
                },
            ),
            ..Default::default()
        })
        .insert(Label {
            label_type: LabelType::MovesCount,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(font_size * 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: font_handle,
                    font_size,
                    color: Color::BLACK,
                },
            ),
            ..Default::default()
        })
        .insert(Label {
            label_type: LabelType::Fps,
        });
}
