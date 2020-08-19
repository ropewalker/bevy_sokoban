mod gameplay;
mod map;

pub use self::gameplay::*;
pub use self::map::*;
use crate::events::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Default)]
pub struct KeyboardState {
    pub event_reader: EventReader<KeyboardInput>,
}

#[derive(Default)]
pub struct EventListenerState {
    pub event_reader: EventReader<GameEvent>,
}

pub struct TileSize(pub f32);

impl Default for TileSize {
    fn default() -> Self {
        TileSize(32.0)
    }
}
