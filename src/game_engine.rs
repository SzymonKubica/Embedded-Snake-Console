use oorandom::Rand32;

use crate::common::BOARD_SIZE;
use crate::libs::time_util::millis;
use crate::traits::{Runnable, ControllerListener, View};
use crate::user_interface as UI;

use crate::internal_representation::controller_input::ControllerInput;
use crate::internal_representation::game_state::{GameState, OperationMode, GameSpeed};
use crate::internal_representation::game_board::{GameBoard, BoardCell};
use crate::internal_representation::map::Map;
use crate::internal_representation::snake::Snake;
use crate::internal_representation::point::Point;
use crate::internal_representation::direction::Direction;

pub struct GameEngine<'a> {
    state: GameState,
    board: GameBoard,
    snake: Snake,
    map: Map,
    generator: Rand32,
    controller_input: ControllerInput,
    view: &'a mut dyn View,
}

impl<'a> Runnable for GameEngine<'a> {
    fn run_once(&mut self) -> () {
        match self.state.mode {
            OperationMode::GameRunning  => self.run_game(),
            OperationMode::InMenu       => self.run_menu(),
            OperationMode::SelectingMap => self.run_map_menu(),
        }
        self.view.run_once();
    }
}

impl<'a> ControllerListener for GameEngine<'a> {
    fn on_input(&mut self, input: ControllerInput) {
        match self.state.mode {
            OperationMode::InMenu => self.controller_input = input,
            _                     => self.override_direction_if_set(input),
        }
    }
}

impl<'a> GameEngine<'a> {
    pub fn new(view: &'a mut dyn View, seed: u16) -> GameEngine {
        GameEngine {
            state: GameState::new(),
            board: GameBoard::default(),
            snake: Snake::new(),
            map: Map::new(),
            generator: oorandom::Rand32::new(seed as u64),
            controller_input: ControllerInput::default(),
            view,
        }
    }

    fn run_game(&mut self) {
        if self.controller_input.toggle_signal {
           self.end_game();
           return;
        }

        if self.state.is_time_for_next_move() {
            self.state.register_move_at(millis());
            self.make_move();
            self.view.update(self.board.get_screen());
        }
    }

    fn run_menu(&mut self) {
        if self.controller_input.toggle_signal {
            self.start_game();
            self.controller_input.reset_signal();
            return;
        }

        match self.controller_input.direction {
            Direction::Left        => self.select_map(),
            Direction::Up          => self.set_speed(GameSpeed::Slow),
            Direction::Right       => self.set_speed(GameSpeed::Normal),
            Direction::Down        => self.set_speed(GameSpeed::Fast),
            Direction::NoDirection => self.print_score(),
        }
    }

    fn run_map_menu(&mut self) {
        if self.controller_input.toggle_signal {
            self.start_game();
            self.controller_input.reset_signal();
            return;
        }

        if !self.map.is_time_for_interaction() {
            return;
        }

        self.map.register_interaction_at(millis());

        match self.controller_input.direction {
            Direction::Right => self.state.return_to_menu(),
            Direction::Up    => self.map.get_next(),
            Direction::Down  => self.map.get_previous(),
            _                => (),
        }

        self.controller_input = ControllerInput::default();

        self.board = GameBoard::new(self.map.get_current_map());
        self.view.update(self.map.print_current_map());
    }

    fn start_game(&mut self) {
        self.board = GameBoard::new(self.map.get_current_map());
        self.board.add_snake_segment(self.snake.head);
        self.spawn_apple();
        self.state.start();
        self.view.update(self.board.get_screen());
    }

    fn end_game(&mut self) {
        self.snake = Snake::new();
        self.board.reset();
        self.state.return_to_menu();
        self.controller_input = ControllerInput::default();
    }

    fn make_move(&mut self) {
        self.snake.change_direction(self.controller_input.direction);

        let cell_ahead = self.snake.look_ahead();

        if !self.board.is_within_bounds(cell_ahead) {
            self.use_grace();
            return;
        }

        match self.board.read_board_at(cell_ahead) {
            BoardCell::Empty                       => self.move_snake_forward(),
            BoardCell::Apple                       => self.eat_apple(),
            BoardCell::Snake | BoardCell::Obstacle => self.use_grace(),
        };
    }

    fn use_grace(&mut self) {
        if self.state.is_grace_pending {
            self.end_game();
        } else {
            self.state.is_grace_pending = true;
        }
    }

    fn move_snake_forward(&mut self) {
        self.move_snake_head();
        self.board.erase_entry(self.snake.advance_tail());
    }

    // When eating an apple, we don't erase the cell occupied by the snake's
    // tail which effectively makes the snake grow.
    fn eat_apple(&mut self) {
        self.move_snake_head();
        self.state.score += 1;

        if self.state.score as usize == self.map.get_max_score() {
            // Terminate if the player wins by filling the whole board.
            return self.end_game();
        }

        self.spawn_apple();
    }

    fn move_snake_head(&mut self) {
        self.state.reset_grace();
        self.snake.move_forward();
        self.board.add_snake_segment(self.snake.head);
    }

    fn spawn_apple(&mut self) {
        loop {
            let apple_x = self.generator.rand_range(0..BOARD_SIZE as u32) as i8;
            let apple_y = self.generator.rand_range(0..BOARD_SIZE as u32) as i8;

            let point = Point::new(apple_x, apple_y);

            if self.board.read_board_at(point) == BoardCell::Empty {
                self.board.add_apple(point);
                break;
            }
        }
    }

    fn override_direction_if_set(&mut self, input: ControllerInput) {
        self.controller_input.toggle_signal = input.toggle_signal;

        // When in game we don't overwrite the direction if it is not set
        // because we don't want to lose user input between steps, we want to
        // read in the direction selection once per snake move and have it
        // persist even if during the subsequent analog stick readings during
        // that move the controller returns NoDirection.
        match input.direction {
            Direction::NoDirection => (), // Don't override when no input
            _ => self.controller_input.direction = input.direction,
        };
    }

    fn select_map(&mut self) {
        self.state.mode = OperationMode::SelectingMap;
        self.view.update(UI::print_up_down_arrows());
    }

    fn print_score(&mut self) {
        let score = self.state.score;

        let screen = if score == 0 {
            UI::print_selection_arrows()
        } else if score as usize == self.map.get_max_score() {
            UI::print_trophy()
        } else {
            UI::print_score(score)
        };

        self.view.update(screen);
    }

    fn set_speed(&mut self, speed: GameSpeed) {
        self.state.game_speed = speed;
        self.view.update(UI::print_speed(speed))
    }
}
