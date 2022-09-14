pub const BOARD_SIZE: usize = 8;

// The time between two steps of the snake,
// effectively controlling the speed of the game. (miliseconds)
pub const SNAKE_MOVE_INTERVAL: u32 = 250;

// The time between calls to controller.get_input(). Controlls the
// responsiveness of the controlls. (miliseconds)
pub const CONTROLLER_POLLING_INTERVAL: u32 = 100;
