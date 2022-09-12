use crate::mvc::{Direction, Model, View, Task};
use crate::internal_representation::Snake;

pub const BOARD_DIMENSION: usize = 8;

// Constants representing game objects on a board.
pub const SNAKE_SEGMENT: u8 = 1;
pub const APPLE: u8 = 2;

// The first column of the matrix doesn't work, hence we restrict the x range.
pub const X_LOWER_BOUND: u8 = 1;
pub const X_UPPER_BOUND: u8 = 7;
pub const Y_LOWER_BOUND: u8 = 0;
pub const Y_UPPER_BOUND: u8 = 7;

pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    game_board: [[u8; BOARD_DIMENSION]; BOARD_DIMENSION], // The board is an 8x8 matrix
    score: u8,
    snake: Snake,
    chosen_direction: Direction
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View) -> GameEngine {
        GameEngine {
            view,
            game_board: initialize_board(),
            score: 0,
            snake: Snake::new(),
            chosen_direction: Direction::NoDirection }
    }

    fn generate_apple(&mut self) {
        loop {
            let apple_x = generate_random_within(BOARD_DIMENSION);
            let apple_y = generate_random_within(BOARD_DIMENSION);

            if self.is_within_bounds((apple_x, apple_y)) {
                self.game_board[apple_y as usize][apple_x as usize] = APPLE;
                return;
            }

        }
    }


    fn is_within_bounds(&self, (x, y): (u8, u8)) -> bool {
        X_LOWER_BOUND <= x && x <= X_UPPER_BOUND &&
        Y_LOWER_BOUND <= y && y <= Y_UPPER_BOUND
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: Direction) {
        self.chosen_direction = input;
    }
}

impl<'a> Task for GameEngine<'a> {
    fn run(&mut self) -> () {
        self.view.run();
    }
}

fn generate_random_within(range: u8) -> u8 {
    rand:
}

pub fn initialize_board() -> [[u8; 8]; 8] {
    let row: [u8; 8] = Default::default();
    let mut matrix: [[u8; 8]; 8] = Default::default();

    for i in 0..8_usize {
        matrix[i] = row;
    }

    matrix
}

