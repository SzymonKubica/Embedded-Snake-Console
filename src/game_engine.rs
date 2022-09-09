use crate::mvc::{Direction, Model, View, Task};
use crate::internal_representation::Snake;


pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    game_board: [[u8; 8]; 8], // The board is an 8x8 matrix
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
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: Direction) {
        self.chosen_direction = input;
    }
}

impl<'a> Task for GameEngine<'a> {
    fn run(&mut self) -> () {
        self.view.update(self.game_board);
        self.view.run();
    }
}

pub fn initialize_board() -> [[u8; 8]; 8] {
    let row: [u8; 8] = Default::default();
    let mut matrix: [[u8; 8]; 8] = Default::default();

    for i in 0..8_usize {
        matrix[i] = row;
    }

    matrix
}

