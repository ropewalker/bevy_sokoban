use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn event_listener_system(
    mut state: ResMut<EventListenerState>,
    mut events: ResMut<Events<GameEvent>>,
    asset_server: Res<AssetServer>,
    audio_output: Res<AudioOutput>,
    mut box_spot_query: Query<(&BoxSpot, &Position)>,
    box_query: Query<(Entity, &Box, &Position)>,
) {
    let mut new_events = Vec::new();

    for event in state.event_reader.iter(&events) {
        match event {
            GameEvent::PlayerHitObstacle => {
                let sound = asset_server.load("assets/sounds/wall.mp3").unwrap();
                audio_output.play(sound);
            }
            GameEvent::EntityMoved(id) => {
                let entity = Entity::new(id.0);

                if let Ok(the_box) = box_query.get::<Box>(entity) {
                    let box_spots_with_positions = box_spot_query
                        .iter()
                        .iter()
                        .map(|t| (*t.1, *t.0))
                        .collect::<HashMap<_, _>>();

                    if let Ok(box_position) = box_query.get::<Position>(entity) {
                        // Check if there is a spot on this position, and if there
                        // is if it's the correct or incorrect type
                        if let Some(box_spot) = box_spots_with_positions.get(&box_position) {
                            new_events.push(GameEvent::BoxPlacedOnSpot(IsCorrectSpot(
                                box_spot.colour == the_box.colour,
                            )));
                        }
                    }
                }
            }
            GameEvent::BoxPlacedOnSpot(IsCorrectSpot(is_correct_spot)) => {
                let sound = asset_server
                    .load(if *is_correct_spot {
                        "assets/sounds/correct.mp3"
                    } else {
                        "assets/sounds/incorrect.mp3"
                    })
                    .unwrap();
                audio_output.play(sound);
            }
        }
    }

    for event in new_events {
        events.send(event);
    }
}
