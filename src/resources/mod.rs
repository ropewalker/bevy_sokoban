// 外に対しての公開を定義する

mod gameplay;
mod map;

pub use self::gameplay::*;
pub use self::map::*;
use bevy::prelude::*;

pub struct TileSize(pub f32);

impl Default for TileSize {
    fn default() -> Self {
        TileSize(32.0)
    }
}

#[derive(Default)]
pub struct SoundHandles {
    pub handles: Vec<HandleUntyped>,
}