pub const BOARD_SIZE: usize = 8;
pub const MAX_SCORE: usize = BOARD_SIZE * BOARD_SIZE - 1;

// The time between two steps of the snake,
// effectively controlling the speed of the game. (miliseconds)
pub const SNAKE_MOVE_INTERVAL_FAST: u32 = 125;
pub const SNAKE_MOVE_INTERVAL_NORMAL: u32 = 250;
pub const SNAKE_MOVE_INTERVAL_SLOW: u32 = 500;

// The time between calls to controller.get_input(). Controlls the
// responsiveness of the controlls. (miliseconds)
pub const CONTROLLER_POLLING_INTERVAL: u32 = 100;
