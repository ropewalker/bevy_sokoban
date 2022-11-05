use crate::resources::*;
use bevy::ecs::component::Component;
use bevy::prelude::{Res, Timer, Transform, Vec3};
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Component)]
pub struct Position {
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
        (-(position.y as f32) + (map.height - 1) as f32 / 2.0) * tile_size.0,
        z,
    ))
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;

        Ok(())
    }
}

#[derive(Debug, Component)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Movable;

#[derive(Component)]
pub struct Immovable;

pub enum LabelType {
    MovesCount,
    GameplayState,
    Fps,
}

#[derive(Component)]
pub struct Label {
    pub label_type: LabelType,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);
