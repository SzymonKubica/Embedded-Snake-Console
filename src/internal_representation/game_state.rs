use crate::libs::time_util::millis;
use crate::common::SNAKE_MOVE_INTERVAL;

pub struct GameState {
    pub score: u8,
    pub last_move_timestamp: u32,
    pub is_active: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { score: 0, last_move_timestamp: millis(), is_active: false }
    }

    pub fn is_time_for_next_move(&self) -> bool {
        millis() - self.last_move_timestamp >= SNAKE_MOVE_INTERVAL
    }

    pub fn register_move_at(&mut self, time: u32) {
        self.last_move_timestamp = time;
    }
}
