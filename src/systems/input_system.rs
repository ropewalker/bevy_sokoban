use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut gameplay: ResMut<Gameplay>,
    mut events: ResMut<Events<GameEvent>>,
    mut player_position_query: Query<(&Player, &Position)>,
    mut movables_query: Query<(Entity, &mut Movable, &Position)>,
    mut immovables_query: Query<(Entity, &Immovable, &Position)>,
) {
    let mut to_move = HashMap::new();

    for (_player, position) in &mut player_position_query.iter() {
        let direction = if keyboard_input.just_pressed(KeyCode::Up) {
            Some(MoveDirection::Up)
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            Some(MoveDirection::Down)
        } else if keyboard_input.just_pressed(KeyCode::Left) {
            Some(MoveDirection::Left)
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            Some(MoveDirection::Right)
        } else {
            None
        };

        if let Some(direction) = direction {
            let mov: HashMap<(usize, usize), u32> = movables_query
                .iter()
                .iter()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();
            let immov: HashMap<(usize, usize), u32> = immovables_query
                .iter()
                .iter()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

            let (start, end, is_x) = match direction {
                MoveDirection::Up => (position.y, 0, false),
                MoveDirection::Down => (position.y, map.height, false),
                MoveDirection::Left => (position.x, 0, true),
                MoveDirection::Right => (position.x, map.width, true),
            };

            let range = if start < end {
                (start..=end).collect::<Vec<_>>()
            } else {
                (end..=start).rev().collect::<Vec<_>>()
            };

            for x_or_y in range {
                let pos = if is_x {
                    (x_or_y, position.y)
                } else {
                    (position.x, x_or_y)
                };

                if let Some(id) = mov.get(&pos) {
                    to_move.insert(*id, direction);
                } else if immov.contains_key(&pos) {
                    to_move.clear();
                    events.send(GameEvent::PlayerHitObstacle);
                } else {
                    break;
                }
            }
        }
    }

    if !to_move.is_empty() {
        gameplay.moves_count += 1;
    }

    for (entity, mut movable, _position) in &mut movables_query.iter() {
        movable.direction = to_move.remove(&entity.id());
    }
}
