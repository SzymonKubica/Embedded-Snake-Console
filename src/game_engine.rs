use rand_chacha::ChaCha8Rng;
use rand::Rng;

use crate::common::BOARD_SIZE;
use crate::internal_representation::point::Point;
use crate::mvc::{Task, Model, Direction, View, ControllerInput};
use crate::internal_representation::snake::Snake;
use crate::internal_representation::game_board::{GameBoard, BoardCell};

pub const FRAMES_BETWEEN_MOVES: i32 = 35;

pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    game_board: GameBoard,
    score: u8,
    snake: Snake,
    controller_input: ControllerInput,
    frames_from_last_move: i32,
    generator: ChaCha8Rng,
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View, generator: ChaCha8Rng) -> GameEngine {
        GameEngine {
            view,
            game_board: GameBoard::new(),
            score: 0,
            snake: Snake::new(),
            controller_input: ControllerInput::new(false, Direction::NoDirection),
            frames_from_last_move: 0,
            generator,
        }
    }

    fn generate_apple(&mut self) {
        loop {
            let apple_x = self.generator.gen_range(0..BOARD_SIZE);
            let apple_y = self.generator.gen_range(0..BOARD_SIZE);

            let point = Point::new(apple_x, apple_y);

            if self.game_board.is_within_bounds(point) {
                self.game_board.add_apple(point);
                return;
            }
        }
    }



    fn update_snake_direction(&mut self, new: Direction) {
        /*
         * The controller_input field represents the choice of user input that
         * will take effect on the next turn on the game (one movement of the
         * snake forward). Therefore we want to update it only if there was no
         * choice made previously (i.e. the direction field of the
         * controller input is set to NoDirection)  If however, the input field
         * has already been set and we receive a signal from the controller
         * with NoDirection reading, we don't overwrite the previous setting.
         * That is because the input will be read multiple times per one turn
         * (one movement of a snake forwards).
         */
        let current: Direction = self.controller_input.direction;

        self.controller_input.direction = match (current, new) {
            (_, Direction::NoDirection) => current, // Don't override with no input
            _                           => new, // Curr value not set -> override
        };
    }

    fn update_signal(&mut self, signal: bool) {
        /*
         * The signal represents if the analog stick of the controller has
         * been pressed in order to start the game or end it. If the value of
         * self.controller_signal.toggle_signal is true, it means that there
         * was a press of the button registered which hasn't been handled yet.
         */

        if !self.controller_input.toggle_signal {
            // Update only if not set.
            self.controller_input.toggle_signal = signal;
        }
    }

    fn take_turn(&mut self) {
        let snake_direction = self.snake.direction;
        let new_direction = self.controller_input.direction;

        // We want to change snake's direction only if if is different from its
        // current direction and also is not the opposite because the snake
        // cannot do a U-turn in place and that would cause it to crash into its
        // other segments.
        if snake_direction != new_direction &&
            snake_direction != Direction::get_opposite(new_direction) {
            self.snake.change_direction(new_direction);
        }

        self.snake.move_head();

        if !self.game_board.is_within_bounds(self.snake.head) {
            self.game_over();
        }

        match self.game_board.read_board_at(self.snake.head) {
            BoardCell::Apple => self.eat_apple(),
            BoardCell::SnakeSegment => self.game_over(),
            BoardCell::Empty => self.move_snake_forward(),
        };
    }

    fn game_over(&mut self) {
        self.game_board = GameBoard::new();
        self.snake = Snake::new();
        self.controller_input = ControllerInput::default()
    }

    fn move_snake_forward(&mut self) {
    }

    fn eat_apple(&self) {
        todo!()
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: ControllerInput) {
        self.update_snake_direction(input.direction);
        self.update_signal(input.toggle_signal);
    }
}

impl<'a> Task for GameEngine<'a> {
    fn run(&mut self) -> () {
        self.view.run();
        self.frames_from_last_move += 1;
        if self.frames_from_last_move == FRAMES_BETWEEN_MOVES {
            self.take_turn();
        }
    }
}

