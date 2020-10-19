use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn input_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    map: Res<Map>,
    mut gameplay: ResMut<Gameplay>,
    mut events: ResMut<Events<GameEvent>>,
    mut player_position_query: Query<(Entity, &Player, &mut Position)>,
    mut movables_query: Query<Without<Player, (Entity, &Movable, &mut Position)>>,
    mut immovables_query: Query<(Entity, &Immovable, &Position)>,
) {
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

        for (entity, _player, mut position) in &mut player_position_query.iter() {
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
                    to_move.insert(*id);
                } else if immov.contains_key(&pos) {
                    to_move.clear();
                    events.send(GameEvent::PlayerHitObstacle);
                    break;
                } else {
                    do_move(&entity, &mut position, &direction, &mut events);
                    gameplay.moves_count += 1;

                    break;
                }
            }
        }

        for (entity, _movable, mut position) in &mut movables_query.iter() {
            if to_move.remove(&entity.id()) {
                do_move(&entity, &mut position, &direction, &mut events);
            }
        }
    }
}

fn do_move(
    entity: &Entity,
    position: &mut Mut<Position>,
    direction: &MoveDirection,
    events: &mut ResMut<Events<GameEvent>>,
) {
    match direction {
        MoveDirection::Up => position.y -= 1,
        MoveDirection::Down => position.y += 1,
        MoveDirection::Left => position.x -= 1,
        MoveDirection::Right => position.x += 1,
    }

    events.send(GameEvent::EntityMoved(EntityId(entity.id())));
}
