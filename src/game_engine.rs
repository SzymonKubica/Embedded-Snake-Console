use crate::concurrency::SchedulerTask;
use crate::controller::{Direction, ControllerListener};


pub struct GameEngine {
    chosen_direction: Direction
}

impl GameEngine {
    pub fn new() -> GameEngine {
        GameEngine { chosen_direction: Direction::NoDirection }
    }
}

impl ControllerListener for GameEngine {
    fn on_input(&mut self, input: Direction) {
        self.chosen_direction = input;
    }
}

impl SchedulerTask for GameEngine {
    fn run_task(&mut self, microseconds: u32) -> () {

    }
}
