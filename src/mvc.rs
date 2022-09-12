use crate::libs::time_util::millis;


pub trait Task {
    fn run(&mut self) -> ();
}

pub trait TimedRunnable : Task {
    fn run_for(&mut self, microseconds: u32) -> ();
}

pub trait Model: TimedRunnable {
    fn on_input(&mut self, input: ControllerInput) -> ();
}

pub trait View: TimedRunnable {
    fn update(&mut self, game_board: [[u8; 8]; 8]) -> ();

}

pub trait Controller {
    fn read_input(&mut self) -> ControllerInput;
}

impl<T> TimedRunnable for T where T: Task {
    fn run_for(&mut self, miliseconds: u32) -> () {
        let time_slice_start = millis();
        let mut current_time = millis();
        while current_time - time_slice_start < miliseconds {
            self.run();
            current_time = millis();
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    NoDirection
}

impl Direction {
    pub fn get_opposite(direction: Direction) -> Direction {
        match direction {
            Direction::Left        => Direction::Right,
            Direction::Right       => Direction::Left,
            Direction::Up          => Direction::Down,
            Direction::Down        => Direction::Up ,
            Direction::NoDirection => Direction::NoDirection,
        }
    }
}

pub struct ControllerInput {
    pub toggle_signal: bool,
    pub direction: Direction
}

impl ControllerInput {
    pub fn new(toggle_signal: bool, direction: Direction) -> ControllerInput {
        ControllerInput { toggle_signal, direction }
    }
}

impl Default for ControllerInput {
    fn default() -> Self {
        Self { toggle_signal: false, direction: Direction::NoDirection }
    }
}
