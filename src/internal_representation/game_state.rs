use crate::libs::time_util::millis;

pub struct GameState {
    pub score: u8,
    pub last_move_timestamp: u32,
    pub is_active: bool,
}

impl GameState {
    pub fn new() -> GameState {

        GameState { score: 0, last_move_timestamp: millis(), is_active: false }
    }
}
