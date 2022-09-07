pub trait Controller {
    fn get_direction(&mut self) -> Direction;
}

pub trait ControllerListener {
    fn on_input(&mut self, input: Direction) -> ();
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    NoDirection
}
