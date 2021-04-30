use crate::components::*;
use crate::events::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn event_listener(
    mut events_reader: EventReader<GameEvent>,
    // mut events_writer: EventWriter<GameEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    box_spot_query: Query<(&BoxSpot, &Position)>,
    box_query: Query<(Entity, &Box, &Position)>,
){
    // let mut new_events = Vec::new();

    for event in events_reader.iter(){
        match event {
            // 壁にぶつかった時のイベント(音を鳴らす)
            GameEvent::PlayerHitObstacle => {
                let sound = asset_server.get_handle("sounds/wall.mp3");
                audio.play(sound);
            }
            // ものが移動した時のイベント
            GameEvent::EntityMoved(id) => {
                let entity = Entity::new(id.0);

                if let Ok((_, r#box, box_position)) = box_query.get(entity) {
                    let box_spots_with_positions = box_spot_query
                        .iter()
                        .map(|t| (*t.1, *t.0))
                        .collect::<HashMap<_, _>>();

                    // Check if there is a spot on this position, and if there is if it's the
                    // correct or incorrect type
                    // 正しい箱をスポットの位置に置けたか判定
                    if let Some(box_spot) = box_spots_with_positions.get(&box_position) {
                        // 正解と不正解の音を鳴らす
                        let sound = asset_server.get_handle(
                            if box_spot.color == r#box.color{
                                "sounds/correct.mp3"
                            }else {
                            "sounds/incorrect.mp3"
                            });
                        audio.play(sound);
                        // new_events.push(GameEvent::BoxPlacedOnSpot(IsCorrectSpot(
                        //     box_spot.color == r#box.color,
                        // )));
                    }
                }
            }
            // // 正解と不正解の音を鳴らす
            // GameEvent::BoxPlacedOnSpot(IsCorrectSpot(is_correct_spot)) => {
            //     let sound = asset_server.get_handle(if *is_correct_spot {
            //         "sounds/correct.mp3"
            //     }else {
            //         "sounds/incorrect.mp3"
            //     });
            //     audio.play(sound);
            // }
        }
    }

    // // 新しいeventを通知する
    // for event in new_events {
    //     events_writer.send(event);
    // }
}