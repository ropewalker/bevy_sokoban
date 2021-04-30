use std::fmt;
use std::fmt::Display;

#[derive(Default, Eq, PartialEq)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,  //  移動回数
}

// ゲームの状態
#[derive(Eq, PartialEq)]
pub enum GameplayState {
    Playing,
    Won,
}

// デフォルト設定の実装
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt ::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}
