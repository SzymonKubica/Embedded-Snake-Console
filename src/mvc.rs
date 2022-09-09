use crate::time_util::millis;

pub trait Task {
    fn run(&mut self) -> ();
}

pub trait TimedRunnable : Task {
    fn run_for(&mut self, microseconds: u32) -> ();
}

pub trait Model: TimedRunnable {
    fn on_input(&mut self, input: Direction) -> ();
}

pub trait View: TimedRunnable {
    fn display(&self) -> ();
    fn update(&mut self) -> ();
}

pub trait Controller<'a>: TimedRunnable {
    fn get_direction(&mut self) -> Direction;
    fn notify_listener(&mut self, input: Direction) -> ();
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

impl<'a, T: Controller<'a>> Task for T {
    fn run(&mut self) -> () {
        let input: Direction = self.get_direction();
        self.notify_listener(input);
    }
}

#[derive(Copy, Clone)]
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
