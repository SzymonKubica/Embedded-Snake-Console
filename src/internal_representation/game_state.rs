use crate::libs::time_util::millis;
use crate::common::{SNAKE_MOVE_INTERVAL_NORMAL, SNAKE_MOVE_INTERVAL_SLOW, SNAKE_MOVE_INTERVAL_FAST};

pub struct GameState {
    pub score: u8,
    last_move_timestamp: u32,
    pub mode: OperationMode,
    pub game_speed: GameSpeed,
    pub is_grace: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            score: 0,
            last_move_timestamp: millis(),
            mode: OperationMode::InMenu,
            game_speed: GameSpeed::Normal,
            is_grace: false,
        }
    }

    pub fn is_time_for_next_move(&self) -> bool {
        millis() - self.last_move_timestamp >= self.game_speed.move_duration()
    }

    pub fn register_move_at(&mut self, time: u32) {
        self.last_move_timestamp = time;
    }

    pub fn restart(&mut self) {
        self.last_move_timestamp = millis();
        self.score = 0;
        self.mode = OperationMode::GameRunning;
    }

    pub fn reset_grace(&mut self) {
        self.is_grace = false;
    }
}

pub enum OperationMode {
    GameRunning, InMenu, SelectingMap
}

#[derive(Copy, Clone)]
pub enum GameSpeed {
    Slow, Normal, Fast
}

impl GameSpeed {
    pub fn move_duration(&self) -> u32 {
        match self {
            GameSpeed::Slow   => SNAKE_MOVE_INTERVAL_SLOW,
            GameSpeed::Normal => SNAKE_MOVE_INTERVAL_NORMAL,
            GameSpeed::Fast   => SNAKE_MOVE_INTERVAL_FAST,
        }
    }
}

