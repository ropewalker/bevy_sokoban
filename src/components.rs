use crate::resources::*; // src/resources/*
use bevy::prelude::{Res, Transform, Vec3};
use std::fmt;
use std::fmt::Display;

// Clone, Copy, Eq, PartialEq, Hash, Debug のトレイトを標準実装を利用する
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Position{
    pub x: usize,
    pub y: usize,
}

pub fn position_to_translation(
    map: &Res<Map>,
    tile_size: &Res<TileSize>,
    position: &Position,
    z: f32,
) -> Transform {
    Transform::from_translation(Vec3::new(
        (position.x as f32 - (map.width - 1) as f32 / 2.0) * tile_size.0,
        (-(position.y as f32) - (map.height - 1) as f32 / 2.0) * tile_size.0 + 300.0, // 全体の描画位置をずらすオフセット
        z,
    ))
}

// 構造体だけ定義してる
pub struct Wall;

pub struct Player;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum BoxColor {
    Red,
    Blue,
}

// BoxColor構造体のDisplayトレイトの実装
impl Display for BoxColor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })?; // ? は try!と同じ意味

        Ok(())
    }
}

#[derive(Debug)]
pub struct Box {
    pub color: BoxColor,
}

#[derive(Clone, Copy, Debug)]
pub struct BoxSpot {
    pub color: BoxColor,
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Movable;

pub struct Immovable;

pub enum LabelType {
    MovesCount,
    GameplayState,
    FPS,
}

pub struct Label {
    pub label_type: LabelType,
}