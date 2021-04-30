use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn input(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut gameplay: ResMut<Gameplay>,
    mut events: EventWriter<GameEvent>,
    mut player_position_query: Query<(Entity, &Player, &mut Position)>,
    mut movables_query: Query<(Entity, &Movable, &mut Position), Without<Player>>,
    immovables_query: Query<(Entity, &Immovable, &Position), (Without<Movable>, Without<Player>)>,
){
    // ゲームが終了していたら抜ける
    if gameplay.state != GameplayState::Playing {
        return;
    }

    // キーボード操作の入力を受け取る
    let direction = if keyboard_input.just_pressed(KeyCode::Up){
        Some(MoveDirection::Up)
    } else if keyboard_input.just_pressed(KeyCode::Down){
        Some(MoveDirection::Down)
    } else if keyboard_input.just_pressed(KeyCode::Left){
        Some(MoveDirection::Left)
    } else if keyboard_input.just_pressed(KeyCode::Right){
        Some(MoveDirection::Right)
    } else {
        None
    };

    if let Some(direction) = direction {
        let mut to_move = HashSet::new();

        for (entity, _player, mut position) in player_position_query.iter_mut(){
            // 位置から移動可能なentityのIdを取得する連想配列を生成
            let mov: HashMap<(usize, usize), u32> = movables_query
                .iter_mut()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();
            // 位置から移動不可能なentityのIdを取得する連想配列を生成
            let immov: HashMap<(usize, usize), u32> = immovables_query
                .iter()
                .map(|t| ((t.2.x, t.2.y), t.0.id()))
                .collect::<HashMap<_, _>>();

            // キー操作をハンドリング
            let (start, end, is_x) = match direction {
                MoveDirection::Up => (position.y - 1, 0, false),
                MoveDirection::Down => (position.y + 1, map.height, false),
                MoveDirection::Left => (position.x - 1, 0, true),
                MoveDirection::Right => (position.x + 1, map.width, true),
            };

            // キー操作のレンジを取得
            let range = if start < end {
                (start..=end).collect::<Vec<_>>()
            } else {
                (end..=start).rev().collect::<Vec<_>>()
            };

            // キー操作のレンジに合わせて物体を動かす
            for x_or_y in range {
                let pos = if is_x {
                    (x_or_y, position.y)
                } else {
                    (position.x, x_or_y)
                };

                if let Some(id) = mov.get(&pos) {
                    // 移動可能物体の移動を登録
                    to_move.insert(*id);
                } else if immov.contains_key(&pos) {
                    // 移動不可能物体の移動から障害物に当たったイベントを追加
                    to_move.clear();
                    events.send(GameEvent::PlayerHitObstacle);
                    break;
                } else {
                    // 物体の移動イベントを追加
                    r#move(&entity, &mut position, &direction, &mut events);
                    gameplay.moves_count += 1;

                    break;
                }
            }
        }

        for (entity, _movable, mut position) in movables_query.iter_mut(){
            if to_move.remove(&entity.id()){
                r#move(&entity, &mut position, &direction, &mut events);
            }
        }
    }
}

fn r#move(
    entity: &Entity,
    position: &mut Mut<Position>,
    direction: &MoveDirection,
    events: &mut EventWriter<GameEvent>,
){
    match direction {
        MoveDirection::Up => position.y -= 1,
        MoveDirection::Down => position.y += 1,
        MoveDirection::Left => position.x -= 1,
        MoveDirection::Right => position.x += 1,
    }

    events.send(GameEvent::EntityMoved(EntityId(entity.id())));
}