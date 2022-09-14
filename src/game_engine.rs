use oorandom::Rand32;

use crate::common::BOARD_SIZE;
use crate::libs::time_util::millis;
use crate::mvc::{Runnable, Model, View};

use crate::internal_representation::controller_input::ControllerInput;
use crate::internal_representation::direction::Direction;
use crate::internal_representation::game_state::GameState;
use crate::internal_representation::point::Point;
use crate::internal_representation::snake::Snake;
use crate::internal_representation::game_board::{GameBoard, BoardCell};

pub struct GameEngine<'a> {
    state: GameState,
    board: GameBoard,
    snake: Snake,

    generator: Rand32,
    controller_input: ControllerInput,
    view: &'a mut dyn View,
}

impl<'a> Runnable for GameEngine<'a> {
    fn run_once(&mut self) -> () {

        if !self.state.is_active &&
            self.controller_input.toggle_signal {

            self.start_game();
            self.controller_input.reset_signal();
        }

        if self.state.is_active && self.state.is_time_for_next_move() {
            self.state.register_move_at(millis());
            self.make_move();
            self.view.update(self.board.to_screen());
        }

        self.view.run_once();
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: ControllerInput) {
        self.controller_input.toggle_signal = input.toggle_signal;

        match input.direction {
            Direction::NoDirection => (), // Don't override when no input
            _ => self.controller_input.direction = input.direction,
        };
    }
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View, seed: u16) -> GameEngine {
        GameEngine {
            state: GameState::new(),
            board: GameBoard::new(),
            snake: Snake::new(),

            generator: oorandom::Rand32::new(seed as u64),
            controller_input: ControllerInput::default(),
            view
        }
    }

    pub fn start_game(&mut self) {
        self.state = GameState::new();
        self.state.is_active = true;
        self.board.add_snake_segment(self.snake.head);
        self.generate_apple();
        self.view.update(self.board.to_screen());
    }

    fn end_game(&mut self) {
        self.snake = Snake::new();
        self.board = GameBoard::new();
        self.controller_input = ControllerInput::default();
        self.state.is_active = false; // state is not reset to save the score.
    }

    fn make_move(&mut self) {

        if self.state.is_active && self.controller_input.toggle_signal {
           self.end_game();
           return;
        }

        self.snake.change_direction(self.controller_input.direction);
        self.snake.move_head();

        if !self.board.is_within_bounds(self.snake.head) {
            self.end_game();
            return;
        }

        match self.board.read_board_at(self.snake.head) {
            BoardCell::Empty        => self.move_snake_forward(),
            BoardCell::Apple        => self.eat_apple(),
            BoardCell::SnakeSegment => self.end_game(),
        };
    }


    fn move_snake_forward(&mut self) {
        self.board.add_snake_segment(self.snake.head);
        self.board.erase_entry(self.snake.advance_tail());
    }

    // When eating an apple, we don't erase the cell occupied by the snake's
    // tail which effectively makes the snake grow.
    fn eat_apple(&mut self) {
        self.board.add_snake_segment(self.snake.head);
        self.state.score += 1;
        self.generate_apple();
    }

    fn generate_apple(&mut self) {
        loop {
            let apple_x = self.generator.rand_range(0..BOARD_SIZE as u32) as i8;
            let apple_y = self.generator.rand_range(0..BOARD_SIZE as u32) as i8;

            let point = Point::new(apple_x, apple_y);

            if self.board.is_within_bounds(point) &&
               self.board.read_board_at(point) == BoardCell::Empty {

                self.board.add_apple(point);
                break;
            }
        }
    }
}
