use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

type WithoutMovableAndPlayer = (Without<Movable>, Without<Player>);

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut gameplay: ResMut<Gameplay>,
    mut game_event_writer: EventWriter<GameEvent>,
    mut player_position_query: Query<(Entity, &Player, &mut Position)>,
    mut movables_query: Query<(Entity, &Movable, &mut Position), Without<Player>>,
    immovables_query: Query<(Entity, &Immovable, &Position), WithoutMovableAndPlayer>,
) {
    if !keyboard_input.is_changed() {
        return;
    }

    if gameplay.state != GameplayState::Playing {
        return;
    }

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
        let mut to_move = HashSet::new();

        for (entity, _player, mut position) in player_position_query.iter_mut() {
            let mov: HashMap<(usize, usize), u32> = movables_query
                .iter_mut()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();
            let immov: HashMap<(usize, usize), u32> = immovables_query
                .iter()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

            let (start, end, is_x) = match direction {
                MoveDirection::Up => (position.y - 1, 0, false),
                MoveDirection::Down => (position.y + 1, map.height, false),
                MoveDirection::Left => (position.x - 1, 0, true),
                MoveDirection::Right => (position.x + 1, map.width, true),
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
                    to_move.insert(id.to_owned());
                } else if immov.contains_key(&pos) {
                    to_move.clear();
                    game_event_writer.send(GameEvent::PlayerHitObstacle);
                    break;
                } else {
                    r#move(&entity, &mut position, &direction, &mut game_event_writer);
                    gameplay.moves_count += 1;

                    break;
                }
            }
        }

        for (entity, _movable, mut position) in movables_query.iter_mut() {
            if to_move.remove(&entity.id()) {
                r#move(&entity, &mut position, &direction, &mut game_event_writer);
            }
        }
    }
}

fn r#move(
    entity: &Entity,
    position: &mut Mut<Position>,
    direction: &MoveDirection,
    game_event_writer: &mut EventWriter<GameEvent>,
) {
    match direction {
        MoveDirection::Up => position.y -= 1,
        MoveDirection::Down => position.y += 1,
        MoveDirection::Left => position.x -= 1,
        MoveDirection::Right => position.x += 1,
    }

    game_event_writer.send(GameEvent::EntityMoved(EntityId(entity.id())));
}
