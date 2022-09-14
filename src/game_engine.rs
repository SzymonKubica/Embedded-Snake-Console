use oorandom::Rand32;

use crate::common::{BOARD_SIZE, SNAKE_MOVE_INTERVAL};
use crate::internal_representation::controller_input::ControllerInput;
use crate::internal_representation::direction::Direction;
use crate::internal_representation::game_state::GameState;
use crate::internal_representation::point::Point;
use crate::libs::time_util::millis;
use crate::mvc::{Runnable, Model, View};
use crate::internal_representation::snake::Snake;
use crate::internal_representation::game_board::{GameBoard, BoardCell};

pub struct GameEngine<'a> {
    view: &'a mut dyn View,
    board: GameBoard,
    game_state: GameState,
    snake: Snake,
    controller_input: ControllerInput,
    generator: Rand32,
}

impl<'a> Runnable for GameEngine<'a> {
    fn run(&mut self) -> () {

        if self.controller_input.toggle_signal &&
            !self.game_state.is_active {

            self.start_game();
            self.controller_input.toggle_signal = false;
        }

        if self.game_state.is_active {
            let now = millis();
            if now - self.game_state.last_move_timestamp >= SNAKE_MOVE_INTERVAL {
                self.take_turn();
                self.game_state.last_move_timestamp = now;
            }
        }
        self.view.update(self.board.to_screen());
        self.view.run();
    }
}

impl<'a> Model for GameEngine<'a> {
    fn on_input(&mut self, input: ControllerInput) {
        self.update_snake_direction(input.direction);
        self.update_signal(input.toggle_signal);
    }
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View, seed: u16) -> GameEngine {
        GameEngine {
            view,
            board: GameBoard::new(),
            game_state: GameState::new(),
            snake: Snake::new(),
            controller_input: ControllerInput::default(),
            generator: oorandom::Rand32::new(seed as u64)
        }
    }

    fn generate_apple(&mut self) {
        loop {
            let apple_x = self.generator.rand_range(0..BOARD_SIZE as u32);
            let apple_y = self.generator.rand_range(0..BOARD_SIZE as u32);

            let point = Point::new(apple_x as i8, apple_y as i8);

            if self.board.is_within_bounds(point) &&
                self.board.read_board_at(point) == BoardCell::Empty {
                self.board.add_apple(point);
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

        if self.game_state.is_active
            && self.controller_input.toggle_signal {

            self.game_over();
            return;
        }


        let snake_direction = self.snake.direction;
        let new_direction = self.controller_input.direction;

        // We want to change snake's direction only if if is different from its
        // current direction and also is not the opposite because the snake
        // cannot do a U-turn in place and that would cause it to crash into its
        // other segments.
        if snake_direction != new_direction &&
            new_direction != Direction::NoDirection &&
            snake_direction != Direction::get_opposite(new_direction) {
            self.snake.change_direction(new_direction);
        }


        self.snake.move_head();

        if !self.board.is_within_bounds(self.snake.head) {
            self.game_over();
            return;
        }

        match self.board.read_board_at(self.snake.head) {
            BoardCell::Apple => self.eat_apple(),
            BoardCell::SnakeSegment => self.game_over(),
            BoardCell::Empty => self.move_snake_forward(),
        };
    }

    fn game_over(&mut self) {
        self.board = GameBoard::new();
        self.snake = Snake::new();
        self.controller_input = ControllerInput::default();
        self.game_state.is_active = false;
    }

    fn move_snake_forward(&mut self) {
        self.board.add_snake_segment(self.snake.head);
        self.board.erase_entry(self.snake.move_tail());
    }

    fn eat_apple(&mut self) {
        self.game_state.score += 1;
        self.board.add_snake_segment(self.snake.head);
        // We don't erase the cell occupied by the snake's tail which
        // effectively makes the snake grow.
        self.generate_apple();
    }

    pub fn start_game(&mut self) {
        self.game_state = GameState::new();
        self.game_state.is_active = true;
        self.board.add_snake_segment(self.snake.head);
        self.generate_apple();
    }
}
