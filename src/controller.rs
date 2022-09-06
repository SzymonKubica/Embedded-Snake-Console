pub trait Controller {
    fn get_direction(&self) -> Direction;
}

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    NO_DIRECTION
}
