pub const FRAMES_BETWEEN_MOVES: i32 = 35;

pub struct GameState {
    pub score: u8,
    pub frames_from_last_move: i32,
    pub is_game_active: bool,
}

impl GameState {
    pub fn new() -> GameState {

        GameState { score: 0, frames_from_last_move: 0, is_game_active: false }
    }
}
